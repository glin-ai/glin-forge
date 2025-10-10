use anyhow::Result;
use clap::Args;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// Path to clean
    #[arg(default_value = ".")]
    pub path: String,

    /// Remove artifacts directory
    #[arg(long)]
    pub artifacts: bool,

    /// Remove target directory
    #[arg(long)]
    pub target: bool,

    /// Remove types directory
    #[arg(long)]
    pub types: bool,

    /// Clean all (equivalent to --artifacts --target --types)
    #[arg(long)]
    pub all: bool,

    /// Clean all contracts in workspace
    #[arg(long)]
    pub workspace: bool,
}

pub async fn execute(args: CleanArgs) -> Result<()> {
    if args.workspace {
        return clean_workspace(&args).await;
    }

    clean_single_directory(&args)
}

/// Clean a single directory
fn clean_single_directory(args: &CleanArgs) -> Result<()> {
    let base_path = PathBuf::from(&args.path);

    println!("{}", "Cleaning build artifacts...".cyan().bold());
    println!();

    let mut cleaned = Vec::new();
    let mut errors = Vec::new();

    // Determine what to clean
    let clean_artifacts =
        args.all || args.artifacts || (!args.target && !args.types && !args.artifacts);
    let clean_target = args.all || args.target;
    let clean_types = args.all || args.types;

    // Clean artifacts/
    if clean_artifacts {
        let artifacts_dir = base_path.join("artifacts");
        if artifacts_dir.exists() {
            match remove_dir_recursive(&artifacts_dir) {
                Ok(size) => {
                    println!("{} Removed artifacts/ ({})", "✓".green(), format_size(size));
                    cleaned.push(("artifacts/", size));
                }
                Err(e) => {
                    println!("{} Failed to remove artifacts/: {}", "✗".red(), e);
                    errors.push(("artifacts/", e.to_string()));
                }
            }
        }
    }

    // Clean target/
    if clean_target {
        let target_dir = base_path.join("target");
        if target_dir.exists() {
            match remove_dir_recursive(&target_dir) {
                Ok(size) => {
                    println!("{} Removed target/ ({})", "✓".green(), format_size(size));
                    cleaned.push(("target/", size));
                }
                Err(e) => {
                    println!("{} Failed to remove target/: {}", "✗".red(), e);
                    errors.push(("target/", e.to_string()));
                }
            }
        }
    }

    // Clean types/
    if clean_types {
        let types_dir = base_path.join("types");
        if types_dir.exists() {
            match remove_dir_recursive(&types_dir) {
                Ok(size) => {
                    println!("{} Removed types/ ({})", "✓".green(), format_size(size));
                    cleaned.push(("types/", size));
                }
                Err(e) => {
                    println!("{} Failed to remove types/: {}", "✗".red(), e);
                    errors.push(("types/", e.to_string()));
                }
            }
        }
    }

    println!();

    if cleaned.is_empty() {
        println!("{} No directories to clean", "ℹ".blue());
    } else {
        let total_size: u64 = cleaned.iter().map(|(_, size)| size).sum();
        println!(
            "{} Cleaned {} director{}, freed {}",
            "✓".green().bold(),
            cleaned.len(),
            if cleaned.len() == 1 { "y" } else { "ies" },
            format_size(total_size)
        );
    }

    if !errors.is_empty() {
        println!();
        println!("{} Some directories could not be cleaned:", "⚠".yellow());
        for (name, error) in &errors {
            println!("  • {}: {}", name, error);
        }
        anyhow::bail!("Clean operation incomplete");
    }

    Ok(())
}

/// Clean all contracts in a workspace
async fn clean_workspace(args: &CleanArgs) -> Result<()> {
    let base_path = PathBuf::from(&args.path);

    println!("{}", "Cleaning workspace...".cyan().bold());
    println!();

    let contracts_dir = base_path.join("contracts");

    if !contracts_dir.exists() {
        anyhow::bail!(
            "No contracts directory found. Expected at: {}",
            contracts_dir.display()
        );
    }

    // Clean each contract
    let mut cleaned_count = 0;
    let mut failed = Vec::new();

    for entry in fs::read_dir(&contracts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let contract_name = path.file_name().unwrap().to_string_lossy();
            println!("Cleaning {}...", contract_name.bold());

            let clean_args = CleanArgs {
                path: path.to_string_lossy().to_string(),
                artifacts: args.artifacts,
                target: args.target,
                types: args.types,
                all: args.all,
                workspace: false,
            };

            match clean_single_directory(&clean_args) {
                Ok(_) => {
                    cleaned_count += 1;
                    println!();
                }
                Err(e) => {
                    failed.push((contract_name.to_string(), e.to_string()));
                    println!("{} Failed: {}\n", "✗".red(), e);
                }
            }
        }
    }

    // Clean workspace-level directories
    println!("Cleaning workspace root...");

    let mut workspace_cleaned = 0;

    // Clean workspace target/
    if args.all || args.target {
        let target_dir = base_path.join("target");
        if target_dir.exists() {
            match remove_dir_recursive(&target_dir) {
                Ok(size) => {
                    println!(
                        "{} Removed workspace target/ ({})",
                        "✓".green(),
                        format_size(size)
                    );
                    workspace_cleaned += 1;
                }
                Err(e) => {
                    println!("{} Failed to remove workspace target/: {}", "✗".red(), e);
                }
            }
        }
    }

    // Clean workspace artifacts/
    if args.all || args.artifacts {
        let artifacts_dir = base_path.join("artifacts");
        if artifacts_dir.exists() {
            match remove_dir_recursive(&artifacts_dir) {
                Ok(size) => {
                    println!(
                        "{} Removed workspace artifacts/ ({})",
                        "✓".green(),
                        format_size(size)
                    );
                    workspace_cleaned += 1;
                }
                Err(e) => {
                    println!("{} Failed to remove workspace artifacts/: {}", "✗".red(), e);
                }
            }
        }
    }

    println!();
    println!("{}", "=== Clean Summary ===".bold());
    println!("  {} {} contract(s) cleaned", "✓".green(), cleaned_count);
    if workspace_cleaned > 0 {
        println!(
            "  {} {} workspace director{} cleaned",
            "✓".green(),
            workspace_cleaned,
            if workspace_cleaned == 1 { "y" } else { "ies" }
        );
    }

    if !failed.is_empty() {
        println!("  {} {} failed", "✗".red(), failed.len());
        for (name, error) in &failed {
            println!("    • {}: {}", name, error);
        }
        anyhow::bail!("Some contracts failed to clean");
    }

    Ok(())
}

/// Remove a directory recursively and return total bytes freed
fn remove_dir_recursive(path: &Path) -> Result<u64> {
    let size = calculate_dir_size(path)?;
    fs::remove_dir_all(path)?;
    Ok(size)
}

/// Calculate total size of a directory
fn calculate_dir_size(path: &Path) -> Result<u64> {
    let mut total_size = 0u64;

    if path.is_file() {
        return Ok(path.metadata()?.len());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                total_size += entry.metadata()?.len();
            } else if entry_path.is_dir() {
                total_size += calculate_dir_size(&entry_path)?;
            }
        }
    }

    Ok(total_size)
}

/// Format file size to human-readable string
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
