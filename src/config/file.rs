use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Configuration from glinforge.config.ts file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    #[serde(default)]
    pub networks: HashMap<String, super::NetworkConfig>,

    #[serde(default = "default_network")]
    pub default_network: String,

    #[serde(default)]
    pub paths: PathsConfig,

    #[serde(default)]
    pub compiler: CompilerConfig,

    #[serde(default)]
    pub typegen: TypeGenConfig,

    #[serde(default)]
    pub test: TestConfig,

    #[serde(default)]
    pub deployments: HashMap<String, HashMap<String, DeploymentConfig>>,

    #[serde(default)]
    pub vars: HashMap<String, serde_json::Value>,
}

fn default_network() -> String {
    "testnet".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    #[serde(default = "default_contracts_path")]
    pub contracts: String,

    #[serde(default = "default_artifacts_path")]
    pub artifacts: String,

    #[serde(default = "default_types_path")]
    pub types: String,

    #[serde(default = "default_scripts_path")]
    pub scripts: String,

    #[serde(default = "default_tests_path")]
    pub tests: String,

    #[serde(default = "default_cache_path")]
    pub cache: String,
}

fn default_contracts_path() -> String {
    "./contracts".to_string()
}
fn default_artifacts_path() -> String {
    "./artifacts".to_string()
}
fn default_types_path() -> String {
    "./types".to_string()
}
fn default_scripts_path() -> String {
    "./scripts".to_string()
}
fn default_tests_path() -> String {
    "./test".to_string()
}
fn default_cache_path() -> String {
    "./.cache".to_string()
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            contracts: default_contracts_path(),
            artifacts: default_artifacts_path(),
            types: default_types_path(),
            scripts: default_scripts_path(),
            tests: default_tests_path(),
            cache: default_cache_path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    #[serde(default = "default_optimize")]
    pub optimize: bool,

    #[serde(default)]
    pub features: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(default)]
    pub cargo_flags: Vec<String>,

    #[serde(default)]
    pub workspace: bool,
}

fn default_optimize() -> bool {
    true
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            optimize: true,
            features: Vec::new(),
            target: None,
            cargo_flags: Vec::new(),
            workspace: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeGenConfig {
    #[serde(default = "default_auto_generate")]
    pub auto_generate: bool,

    #[serde(default = "default_types_path")]
    pub out_dir: String,

    #[serde(default)]
    pub hooks: bool,

    #[serde(default)]
    pub legacy: bool,

    #[serde(default = "default_style")]
    pub style: String,
}

fn default_auto_generate() -> bool {
    true
}
fn default_style() -> String {
    "interface".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    #[serde(default = "default_framework")]
    pub framework: String,

    #[serde(default = "default_pattern")]
    pub pattern: String,

    #[serde(default = "default_timeout")]
    pub timeout: u64,

    #[serde(default)]
    pub parallel: bool,

    #[serde(default)]
    pub coverage: bool,
}

fn default_framework() -> String {
    "mocha".to_string()
}
fn default_pattern() -> String {
    "test/**/*.test.ts".to_string()
}
fn default_timeout() -> u64 {
    30000
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            framework: default_framework(),
            pattern: default_pattern(),
            timeout: default_timeout(),
            parallel: false,
            coverage: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeploymentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(default)]
    pub args: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,

    #[serde(default)]
    pub verify: bool,

    #[serde(default = "default_wait_for_finalization")]
    pub wait_for_finalization: bool,
}

fn default_wait_for_finalization() -> bool {
    true
}

/// Load configuration from file
pub fn load_config_file(path: Option<&Path>) -> Result<FileConfig> {
    let config_path = if let Some(p) = path {
        p.to_path_buf()
    } else {
        find_config_file()?
    };

    // Check file extension
    let extension = config_path
        .extension()
        .and_then(|e| e.to_str())
        .context("Invalid config file extension")?;

    match extension {
        "ts" => load_typescript_config(&config_path),
        "js" => load_javascript_config(&config_path),
        "json" => load_json_config(&config_path),
        _ => anyhow::bail!(
            "Unsupported config file format: {}. Use .ts, .js, or .json",
            extension
        ),
    }
}

/// Find config file in current directory
fn find_config_file() -> Result<PathBuf> {
    let config_files = [
        "glinforge.config.ts",
        "glinforge.config.js",
        "glinforge.config.json",
        "glin-forge.config.ts",
        "glin-forge.config.js",
        "glin-forge.config.json",
    ];

    for file in &config_files {
        let path = PathBuf::from(file);
        if path.exists() {
            return Ok(path);
        }
    }

    anyhow::bail!("No configuration file found. Create glinforge.config.ts in your project root.")
}

/// Load TypeScript config file
fn load_typescript_config(path: &Path) -> Result<FileConfig> {
    // Use ts-node to execute TypeScript config
    let output = Command::new("node")
        .args([
            "-e",
            &format!(
                r#"
                const tsNode = require('ts-node');
                tsNode.register({{ transpileOnly: true, compilerOptions: {{ module: 'commonjs' }} }});
                const config = require('{}');
                console.log(JSON.stringify(config.default || config));
                "#,
                path.canonicalize()?.display()
            ),
        ])
        .output()
        .context("Failed to execute Node.js. Ensure Node.js and ts-node are installed.")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to load TypeScript config: {}", error);
    }

    let json_str = String::from_utf8(output.stdout)?;
    let config: FileConfig = serde_json::from_str(&json_str)
        .context("Failed to parse configuration from TypeScript file")?;

    Ok(config)
}

/// Load JavaScript config file
fn load_javascript_config(path: &Path) -> Result<FileConfig> {
    let output = Command::new("node")
        .args([
            "-e",
            &format!(
                r#"
                const config = require('{}');
                console.log(JSON.stringify(config.default || config));
                "#,
                path.canonicalize()?.display()
            ),
        ])
        .output()
        .context("Failed to execute Node.js. Ensure Node.js is installed.")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to load JavaScript config: {}", error);
    }

    let json_str = String::from_utf8(output.stdout)?;
    let config: FileConfig = serde_json::from_str(&json_str)
        .context("Failed to parse configuration from JavaScript file")?;

    Ok(config)
}

/// Load JSON config file
fn load_json_config(path: &Path) -> Result<FileConfig> {
    let json_str = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: FileConfig =
        serde_json::from_str(&json_str).context("Failed to parse JSON configuration")?;

    Ok(config)
}

/// Merge file config with default config
pub fn merge_with_defaults(file_config: FileConfig) -> super::ForgeConfig {
    let mut config = super::ForgeConfig::default();

    // Merge networks
    for (name, network) in file_config.networks {
        config.networks.insert(name, network);
    }

    // Set default network
    config.default_network = file_config.default_network;

    config
}

/// Get network from file config
pub fn get_network_from_file(
    file_config: &FileConfig,
    network_name: Option<&str>,
) -> Result<super::NetworkConfig> {
    let name = network_name.unwrap_or(&file_config.default_network);

    file_config
        .networks
        .get(name)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Network '{}' not found in configuration", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FileConfig {
            networks: HashMap::new(),
            default_network: "testnet".to_string(),
            paths: PathsConfig::default(),
            compiler: CompilerConfig::default(),
            typegen: TypeGenConfig::default(),
            test: TestConfig::default(),
            deployments: HashMap::new(),
            vars: HashMap::new(),
        };

        assert_eq!(config.default_network, "testnet");
        assert_eq!(config.paths.contracts, "./contracts");
        assert!(config.compiler.optimize);
    }

    #[test]
    fn test_config_serialization() {
        let config = FileConfig {
            networks: HashMap::new(),
            default_network: "testnet".to_string(),
            paths: PathsConfig::default(),
            compiler: CompilerConfig::default(),
            typegen: TypeGenConfig::default(),
            test: TestConfig::default(),
            deployments: HashMap::new(),
            vars: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: FileConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.default_network, config.default_network);
    }
}
