// Metadata parsing utilities for code generation

use anyhow::Result;
use serde_json::Value as JsonValue;

/// Extract contract name from metadata
pub fn extract_contract_name(abi: &JsonValue) -> Result<String> {
    abi["contract"]["name"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Contract name not found in metadata"))
}

/// Extract contract version from metadata
pub fn extract_contract_version(abi: &JsonValue) -> Option<String> {
    abi["version"]
        .as_str()
        .or_else(|| abi["contract"]["version"].as_str())
        .map(|s| s.to_string())
}

/// Extract all message definitions from metadata
pub fn extract_messages(abi: &JsonValue) -> Result<Vec<MessageInfo>> {
    let messages = abi["spec"]["messages"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Messages not found in metadata"))?;

    messages
        .iter()
        .map(|msg| {
            let label = msg["label"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Message label missing"))?
                .to_string();

            let mutates = msg["mutates"].as_bool().unwrap_or(false);

            let args = msg["args"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|arg| {
                            Some(ArgumentInfo {
                                label: arg["label"].as_str()?.to_string(),
                                type_info: arg["type"].clone(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let return_type = msg["returnType"].clone();

            Ok(MessageInfo {
                label,
                mutates,
                args,
                return_type,
            })
        })
        .collect()
}

/// Extract constructor definitions from metadata
pub fn extract_constructors(abi: &JsonValue) -> Result<Vec<ConstructorInfo>> {
    let constructors = abi["spec"]["constructors"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Constructors not found in metadata"))?;

    constructors
        .iter()
        .map(|ctor| {
            let label = ctor["label"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Constructor label missing"))?
                .to_string();

            let args = ctor["args"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|arg| {
                            Some(ArgumentInfo {
                                label: arg["label"].as_str()?.to_string(),
                                type_info: arg["type"].clone(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            Ok(ConstructorInfo { label, args })
        })
        .collect()
}

/// Information about a contract message (method)
#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub label: String,
    pub mutates: bool,
    pub args: Vec<ArgumentInfo>,
    pub return_type: JsonValue,
}

/// Information about a constructor
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    pub label: String,
    pub args: Vec<ArgumentInfo>,
}

/// Information about a method/constructor argument
#[derive(Debug, Clone)]
pub struct ArgumentInfo {
    pub label: String,
    pub type_info: JsonValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_abi() -> JsonValue {
        serde_json::json!({
            "contract": {
                "name": "MyContract",
                "version": "1.0.0"
            },
            "spec": {
                "constructors": [
                    {
                        "label": "new",
                        "args": [
                            {
                                "label": "initial_value",
                                "type": {
                                    "displayName": ["u32"]
                                }
                            }
                        ]
                    }
                ],
                "messages": [
                    {
                        "label": "get",
                        "mutates": false,
                        "args": [],
                        "returnType": {
                            "type": {
                                "displayName": ["u32"]
                            }
                        }
                    },
                    {
                        "label": "set",
                        "mutates": true,
                        "args": [
                            {
                                "label": "value",
                                "type": {
                                    "displayName": ["u32"]
                                }
                            }
                        ],
                        "returnType": null
                    }
                ]
            }
        })
    }

    #[test]
    fn test_extract_contract_name() {
        let abi = sample_abi();
        let name = extract_contract_name(&abi).unwrap();
        assert_eq!(name, "MyContract");
    }

    #[test]
    fn test_extract_contract_version() {
        let abi = sample_abi();
        let version = extract_contract_version(&abi);
        assert_eq!(version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_extract_messages() {
        let abi = sample_abi();
        let messages = extract_messages(&abi).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].label, "get");
        assert_eq!(messages[0].mutates, false);
        assert_eq!(messages[1].label, "set");
        assert_eq!(messages[1].mutates, true);
    }

    #[test]
    fn test_extract_constructors() {
        let abi = sample_abi();
        let constructors = extract_constructors(&abi).unwrap();
        assert_eq!(constructors.len(), 1);
        assert_eq!(constructors[0].label, "new");
        assert_eq!(constructors[0].args.len(), 1);
    }
}
