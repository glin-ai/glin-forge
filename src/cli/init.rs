use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::Path;
use handlebars::Handlebars;
use serde_json::json;

#[derive(Parser)]
pub struct InitArgs {
    #[arg(default_value = ".")]
    pub path: String,
}

pub async fn execute(args: InitArgs) -> anyhow::Result<()> {
    println!("{}", "Initializing new contract project...".cyan().bold());

    let path = Path::new(&args.path);

    // Check if directory is empty (allow .git, .gitignore)
    if path.exists() {
        let entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name();
                let name_str = name.to_string_lossy();
                !name_str.starts_with(".git")
            })
            .collect();

        if !entries.is_empty() {
            anyhow::bail!("Directory is not empty. Use 'glin-forge new <name>' to create a new project.");
        }
    } else {
        fs::create_dir_all(path)?;
    }

    // Get project name from directory
    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my_contract")
        .to_string();

    println!("  {} Project name: {}", "→".cyan(), project_name);

    // Setup handlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);

    // Convert project name to different formats
    let contract_name = project_name.replace("-", "_");
    let contract_name_pascal = to_pascal_case(&project_name);

    let template_data = json!({
        "project_name": project_name,
        "contract_name": contract_name,
        "contract_name_pascal": contract_name_pascal,
        "author": "Your Name <you@example.com>",
    });

    // Read and render Cargo.toml
    let cargo_toml_template = include_str!("../../templates/erc20/Cargo.toml.hbs");
    let cargo_toml_content = handlebars.render_template(cargo_toml_template, &template_data)?;

    let cargo_toml_path = path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;
    println!("  {} Created: Cargo.toml", "✓".green());

    // Read and render lib.rs
    let lib_rs_template = include_str!("../../templates/erc20/lib.rs.hbs");
    let lib_rs_content = handlebars.render_template(lib_rs_template, &template_data)?;

    let lib_rs_path = path.join("lib.rs");
    fs::write(&lib_rs_path, lib_rs_content)?;
    println!("  {} Created: lib.rs", "✓".green());

    // Create .gitignore
    let gitignore_content = r#"target/
Cargo.lock
**/*.rs.bk
*.pdb
.DS_Store
"#;
    let gitignore_path = path.join(".gitignore");
    fs::write(&gitignore_path, gitignore_content)?;
    println!("  {} Created: .gitignore", "✓".green());

    println!(
        "\n{} Contract project initialized successfully!",
        "✓".green().bold()
    );

    println!("\n{}", "Next steps:".bold());
    println!("  glin-forge build");
    println!("  glin-forge test");
    println!("  glin-forge deploy --network testnet --account alice");

    Ok(())
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
