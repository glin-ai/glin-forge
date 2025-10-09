// Type resolution system for ink! metadata to TypeScript type mapping
//
// This module handles converting ink! smart contract types (represented using scale-info)
// into TypeScript type definitions. It supports all 8 TypeDef variants:
// - Primitive: bool, u8-u256, i8-i256, str, char
// - Composite: Structs with named or unnamed fields
// - Variant: Enums, Option, Result
// - Sequence: Vec<T>
// - Array: [T; N]
// - Tuple: (T1, T2, ...)
// - Compact: Compact<T> (SCALE encoding optimization)
// - BitSequence: BitVec (not commonly used)

use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, HashSet};

/// Represents a TypeScript type
#[derive(Debug, Clone, PartialEq)]
pub enum TypeScriptType {
    /// Primitive types: boolean, number, string, bigint
    Primitive(String),

    /// Interface type with named fields
    Interface {
        name: String,
        fields: Vec<(String, Box<TypeScriptType>)>,
        docs: Vec<String>,
    },

    /// Discriminated union (for enums, Option, Result)
    Union {
        name: String,
        variants: Vec<UnionVariant>,
        docs: Vec<String>,
    },

    /// Array type: T[]
    Array(Box<TypeScriptType>),

    /// Tuple type: [T1, T2, ...]
    Tuple(Vec<TypeScriptType>),

    /// Union of multiple types: T1 | T2 | ...
    Or(Vec<TypeScriptType>),

    /// Optional type: T | null
    Optional(Box<TypeScriptType>),

    /// Type reference by name
    Reference(String),

    /// Any type (fallback)
    Any,
}

/// Variant in a discriminated union
#[derive(Debug, Clone, PartialEq)]
pub struct UnionVariant {
    pub name: String,
    pub fields: Vec<(Option<String>, TypeScriptType)>,
    pub docs: Vec<String>,
}

/// Type resolver that converts ink! types to TypeScript
pub struct TypeResolver {
    /// Type registry from metadata
    types: HashMap<u32, JsonValue>,

    /// Cache of resolved types to avoid infinite recursion
    resolved_cache: HashMap<u32, TypeScriptType>,

    /// Custom type names being resolved (for cycle detection)
    resolving_stack: HashSet<u32>,

    /// Named types (structs, enums) that need separate declarations
    named_types: HashMap<String, TypeScriptType>,
}

impl TypeResolver {
    /// Create a new type resolver from metadata types array
    pub fn new(types_array: &JsonValue) -> Result<Self> {
        let types_vec = types_array
            .as_array()
            .context("Types section must be an array")?;

        let mut types = HashMap::new();
        for type_entry in types_vec {
            let id = type_entry["id"]
                .as_u64()
                .context("Type entry missing id")?
                as u32;
            let type_def = type_entry["type"].clone();
            types.insert(id, type_def);
        }

        Ok(Self {
            types,
            resolved_cache: HashMap::new(),
            resolving_stack: HashSet::new(),
            named_types: HashMap::new(),
        })
    }

    /// Resolve a type ID to a TypeScript type
    pub fn resolve_type(&mut self, type_id: u32) -> Result<TypeScriptType> {
        // Check cache first
        if let Some(cached) = self.resolved_cache.get(&type_id) {
            return Ok(cached.clone());
        }

        // Check for cycles
        if self.resolving_stack.contains(&type_id) {
            return Ok(TypeScriptType::Any);
        }

        self.resolving_stack.insert(type_id);

        let type_def = self.types.get(&type_id)
            .with_context(|| format!("Type ID {} not found in registry", type_id))?
            .clone();

        let result = self.resolve_type_def(&type_def)?;

        self.resolving_stack.remove(&type_id);
        self.resolved_cache.insert(type_id, result.clone());

        Ok(result)
    }

    /// Resolve a type definition
    fn resolve_type_def(&mut self, type_def: &JsonValue) -> Result<TypeScriptType> {
        let path = type_def["path"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let def = &type_def["def"];

        // Check for well-known types by path
        if !path.is_empty() {
            if let Some(ts_type) = self.resolve_well_known_type(&path, type_def)? {
                return Ok(ts_type);
            }
        }

        // Handle by TypeDef variant
        if let Some(primitive) = def.get("primitive").and_then(|v| v.as_str()) {
            return self.resolve_primitive(primitive);
        }

        if let Some(composite) = def.get("composite") {
            return self.resolve_composite(composite, &path);
        }

        if let Some(variant) = def.get("variant") {
            return self.resolve_variant(variant, &path);
        }

        if let Some(sequence) = def.get("sequence") {
            return self.resolve_sequence(sequence);
        }

        if let Some(array) = def.get("array") {
            return self.resolve_array(array);
        }

        if let Some(tuple) = def.get("tuple") {
            return self.resolve_tuple(tuple);
        }

        if let Some(compact) = def.get("compact") {
            return self.resolve_compact(compact);
        }

        if let Some(bit_sequence) = def.get("bitSequence") {
            return self.resolve_bit_sequence(bit_sequence);
        }

        Ok(TypeScriptType::Any)
    }

    /// Resolve well-known types by their path
    fn resolve_well_known_type(
        &mut self,
        path: &[&str],
        type_def: &JsonValue,
    ) -> Result<Option<TypeScriptType>> {
        let path_str = path.join("::");

        match path_str.as_str() {
            "Option" => {
                let params = type_def["params"].as_array();
                if let Some(params) = params {
                    if let Some(inner_type_id) = params.first().and_then(|p| p["type"].as_u64()) {
                        let inner = self.resolve_type(inner_type_id as u32)?;
                        return Ok(Some(TypeScriptType::Optional(Box::new(inner))));
                    }
                }
                return Ok(Some(TypeScriptType::Or(vec![
                    TypeScriptType::Any,
                    TypeScriptType::Primitive("null".to_string()),
                ])));
            }
            "Result" => {
                let params = type_def["params"].as_array();
                if let Some(params) = params {
                    let ok_type = if let Some(ok_id) = params.get(0).and_then(|p| p["type"].as_u64()) {
                        self.resolve_type(ok_id as u32)?
                    } else {
                        TypeScriptType::Any
                    };

                    let err_type = if let Some(err_id) = params.get(1).and_then(|p| p["type"].as_u64()) {
                        self.resolve_type(err_id as u32)?
                    } else {
                        TypeScriptType::Any
                    };

                    return Ok(Some(TypeScriptType::Union {
                        name: "Result".to_string(),
                        variants: vec![
                            UnionVariant {
                                name: "Ok".to_string(),
                                fields: vec![(Some("value".to_string()), ok_type)],
                                docs: vec![],
                            },
                            UnionVariant {
                                name: "Err".to_string(),
                                fields: vec![(Some("error".to_string()), err_type)],
                                docs: vec![],
                            },
                        ],
                        docs: vec![],
                    }));
                }
            }
            "Vec" | "BTreeMap" | "BTreeSet" => {
                // Handle as sequence in resolve_sequence
                return Ok(None);
            }
            "String" => {
                return Ok(Some(TypeScriptType::Primitive("string".to_string())));
            }
            path if path.contains("AccountId") => {
                return Ok(Some(TypeScriptType::Primitive("string".to_string())));
            }
            path if path.contains("Balance") => {
                return Ok(Some(TypeScriptType::Or(vec![
                    TypeScriptType::Primitive("string".to_string()),
                    TypeScriptType::Primitive("number".to_string()),
                    TypeScriptType::Primitive("bigint".to_string()),
                ])));
            }
            path if path.contains("Hash") => {
                return Ok(Some(TypeScriptType::Or(vec![
                    TypeScriptType::Primitive("string".to_string()),
                    TypeScriptType::Reference("Uint8Array".to_string()),
                ])));
            }
            _ => {}
        }

        Ok(None)
    }

    /// Resolve primitive types
    fn resolve_primitive(&self, primitive: &str) -> Result<TypeScriptType> {
        let ts_type = match primitive {
            "bool" => "boolean",
            "char" => "string",
            "str" => "string",
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => "number",
            "u64" | "u128" | "u256" | "i64" | "i128" | "i256" => {
                // Return union type for safety
                return Ok(TypeScriptType::Or(vec![
                    TypeScriptType::Primitive("string".to_string()),
                    TypeScriptType::Primitive("number".to_string()),
                    TypeScriptType::Primitive("bigint".to_string()),
                ]));
            }
            _ => "any",
        };

        Ok(TypeScriptType::Primitive(ts_type.to_string()))
    }

    /// Resolve composite types (structs)
    fn resolve_composite(
        &mut self,
        composite: &JsonValue,
        path: &[&str],
    ) -> Result<TypeScriptType> {
        let fields_array = composite["fields"]
            .as_array()
            .context("Composite type missing fields")?;

        // Check if this is a tuple-like struct (all unnamed fields)
        let is_tuple = fields_array
            .iter()
            .all(|f| f.get("name").is_none() || f["name"].is_null());

        if is_tuple {
            // Treat as tuple
            let mut tuple_types = Vec::new();
            for field in fields_array {
                let type_id = field["type"]
                    .as_u64()
                    .context("Field missing type")?
                    as u32;
                let field_type = self.resolve_type(type_id)?;
                tuple_types.push(field_type);
            }

            // Single unnamed field - unwrap it
            if tuple_types.len() == 1 {
                return Ok(tuple_types.into_iter().next().unwrap());
            }

            return Ok(TypeScriptType::Tuple(tuple_types));
        }

        // Named struct
        let mut fields = Vec::new();
        for field in fields_array {
            let name = field["name"]
                .as_str()
                .context("Named field missing name")?
                .to_string();
            let type_id = field["type"]
                .as_u64()
                .context("Field missing type")?
                as u32;
            let field_type = self.resolve_type(type_id)?;
            fields.push((name, Box::new(field_type)));
        }

        let type_name = if path.is_empty() {
            format!("Struct{}", fields.len())
        } else {
            path.last().unwrap().to_string()
        };

        Ok(TypeScriptType::Interface {
            name: type_name,
            fields,
            docs: vec![],
        })
    }

    /// Resolve variant types (enums)
    fn resolve_variant(
        &mut self,
        variant: &JsonValue,
        path: &[&str],
    ) -> Result<TypeScriptType> {
        let variants_array = variant["variants"]
            .as_array()
            .context("Variant type missing variants")?;

        let mut variants = Vec::new();

        for var in variants_array {
            let name = var["name"]
                .as_str()
                .context("Variant missing name")?
                .to_string();

            let mut fields = Vec::new();

            if let Some(fields_array) = var["fields"].as_array() {
                for (idx, field) in fields_array.iter().enumerate() {
                    let field_name = field["name"]
                        .as_str()
                        .map(|s| s.to_string());

                    let type_id = field["type"]
                        .as_u64()
                        .context("Field missing type")?
                        as u32;
                    let field_type = self.resolve_type(type_id)?;

                    let name = field_name.or_else(|| {
                        if fields_array.len() == 1 {
                            Some("value".to_string())
                        } else {
                            Some(format!("field{}", idx))
                        }
                    });

                    fields.push((name, field_type));
                }
            }

            variants.push(UnionVariant {
                name,
                fields,
                docs: vec![],
            });
        }

        let type_name = if path.is_empty() {
            format!("Enum{}", variants.len())
        } else {
            path.last().unwrap().to_string()
        };

        Ok(TypeScriptType::Union {
            name: type_name,
            variants,
            docs: vec![],
        })
    }

    /// Resolve sequence types (Vec<T>)
    fn resolve_sequence(&mut self, sequence: &JsonValue) -> Result<TypeScriptType> {
        let inner_type_id = sequence["type"]
            .as_u64()
            .context("Sequence missing type")?
            as u32;

        let inner = self.resolve_type(inner_type_id)?;

        // Special case: Vec<u8> -> Uint8Array or string
        if matches!(inner, TypeScriptType::Primitive(ref p) if p == "number") {
            return Ok(TypeScriptType::Or(vec![
                TypeScriptType::Reference("Uint8Array".to_string()),
                TypeScriptType::Primitive("string".to_string()),
            ]));
        }

        Ok(TypeScriptType::Array(Box::new(inner)))
    }

    /// Resolve array types [T; N]
    fn resolve_array(&mut self, array: &JsonValue) -> Result<TypeScriptType> {
        let inner_type_id = array["type"]
            .as_u64()
            .context("Array missing type")?
            as u32;
        let len = array["len"].as_u64().unwrap_or(0);

        let inner = self.resolve_type(inner_type_id)?;

        // Special case: [u8; 32] for hashes -> Uint8Array or string
        if len == 32 && matches!(inner, TypeScriptType::Primitive(ref p) if p == "number") {
            return Ok(TypeScriptType::Or(vec![
                TypeScriptType::Reference("Uint8Array".to_string()),
                TypeScriptType::Primitive("string".to_string()),
            ]));
        }

        // For other fixed arrays, use Array<T>
        Ok(TypeScriptType::Array(Box::new(inner)))
    }

    /// Resolve tuple types
    fn resolve_tuple(&mut self, tuple: &JsonValue) -> Result<TypeScriptType> {
        let type_ids = tuple
            .as_array()
            .context("Tuple must be an array")?;

        if type_ids.is_empty() {
            // Unit type ()
            return Ok(TypeScriptType::Primitive("void".to_string()));
        }

        let mut types = Vec::new();
        for type_id_val in type_ids {
            let type_id = type_id_val.as_u64().context("Invalid type ID")? as u32;
            let resolved = self.resolve_type(type_id)?;
            types.push(resolved);
        }

        Ok(TypeScriptType::Tuple(types))
    }

    /// Resolve compact types
    fn resolve_compact(&mut self, compact: &JsonValue) -> Result<TypeScriptType> {
        let inner_type_id = compact["type"]
            .as_u64()
            .context("Compact missing type")?
            as u32;

        // Compact is a SCALE encoding optimization, doesn't affect TS type
        self.resolve_type(inner_type_id)
    }

    /// Resolve bit sequence types
    fn resolve_bit_sequence(&mut self, _bit_sequence: &JsonValue) -> Result<TypeScriptType> {
        // BitVec types -> Uint8Array
        Ok(TypeScriptType::Reference("Uint8Array".to_string()))
    }

    /// Get all named types that need separate declarations
    pub fn get_named_types(&self) -> &HashMap<String, TypeScriptType> {
        &self.named_types
    }

    /// Format a TypeScript type as a string
    pub fn format_type(&self, ts_type: &TypeScriptType) -> String {
        match ts_type {
            TypeScriptType::Primitive(name) => name.clone(),
            TypeScriptType::Reference(name) => name.clone(),
            TypeScriptType::Array(inner) => {
                format!("{}[]", self.format_type(inner))
            }
            TypeScriptType::Tuple(types) => {
                let formatted = types
                    .iter()
                    .map(|t| self.format_type(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", formatted)
            }
            TypeScriptType::Or(types) => {
                types
                    .iter()
                    .map(|t| self.format_type(t))
                    .collect::<Vec<_>>()
                    .join(" | ")
            }
            TypeScriptType::Optional(inner) => {
                format!("{} | null", self.format_type(inner))
            }
            TypeScriptType::Interface { name, .. } => name.clone(),
            TypeScriptType::Union { name, .. } => name.clone(),
            TypeScriptType::Any => "any".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_primitive_types() {
        let resolver = TypeResolver {
            types: HashMap::new(),
            resolved_cache: HashMap::new(),
            resolving_stack: HashSet::new(),
            named_types: HashMap::new(),
        };

        assert_eq!(
            resolver.resolve_primitive("bool").unwrap(),
            TypeScriptType::Primitive("boolean".to_string())
        );

        assert_eq!(
            resolver.resolve_primitive("u32").unwrap(),
            TypeScriptType::Primitive("number".to_string())
        );

        // u128 should be union type
        match resolver.resolve_primitive("u128").unwrap() {
            TypeScriptType::Or(types) => assert_eq!(types.len(), 3),
            _ => panic!("Expected Or type for u128"),
        }
    }

    #[test]
    fn test_format_type() {
        let resolver = TypeResolver {
            types: HashMap::new(),
            resolved_cache: HashMap::new(),
            resolving_stack: HashSet::new(),
            named_types: HashMap::new(),
        };

        assert_eq!(
            resolver.format_type(&TypeScriptType::Primitive("boolean".to_string())),
            "boolean"
        );

        assert_eq!(
            resolver.format_type(&TypeScriptType::Array(Box::new(
                TypeScriptType::Primitive("number".to_string())
            ))),
            "number[]"
        );

        assert_eq!(
            resolver.format_type(&TypeScriptType::Optional(Box::new(
                TypeScriptType::Primitive("string".to_string())
            ))),
            "string | null"
        );
    }
}
