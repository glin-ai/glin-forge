// Code generation module for TypeScript/JavaScript bindings

pub mod hooks;
pub mod metadata;
pub mod type_resolver;
pub mod types;
pub mod typescript;

// Re-export main functions for convenience
pub use hooks::generate_react_hooks;
pub use metadata::{
    extract_constructors, extract_contract_name, extract_contract_version, extract_messages,
    ArgumentInfo, ConstructorInfo, MessageInfo,
};
pub use type_resolver::{TypeResolver, TypeScriptType, UnionVariant};
pub use types::generate_typescript_types;
pub use typescript::generate_typescript_module;
