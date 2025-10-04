use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::Path;
use handlebars::Handlebars;
use serde_json::json;

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,
    #[arg(short, long, default_value = "erc20")]
    pub template: String,
}

pub async fn execute(args: NewArgs) -> anyhow::Result<()> {
    println!("{}", format!("Creating new contract: {}", args.name).cyan().bold());

    // Validate project name
    if Path::new(&args.name).exists() {
        anyhow::bail!("Directory '{}' already exists", args.name);
    }

    // Get template
    let template_name = args.template.to_lowercase();
    let valid_templates = vec!["erc20", "erc721", "dao"];

    if !valid_templates.contains(&template_name.as_str()) {
        anyhow::bail!(
            "Template '{}' not found. Available templates: {}",
            args.template,
            valid_templates.join(", ")
        );
    }

    println!("  {} Using template: {}", "→".cyan(), template_name);

    // Create project directory
    fs::create_dir_all(&args.name)?;
    println!("  {} Created directory: {}", "✓".green(), args.name);

    // Setup handlebars
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);

    // Convert project name to different formats
    let contract_name = args.name.replace("-", "_");
    let contract_name_pascal = to_pascal_case(&args.name);

    let template_data = json!({
        "project_name": args.name,
        "contract_name": contract_name,
        "contract_name_pascal": contract_name_pascal,
        "author": "Your Name <you@example.com>",
    });

    // Read and render templates based on template_name
    let (cargo_toml_template, lib_rs_template) = match template_name.as_str() {
        "erc20" => (
            include_str!("../../templates/erc20/Cargo.toml.hbs"),
            include_str!("../../templates/erc20/lib.rs.hbs"),
        ),
        "erc721" => (
            include_str!("../../templates/erc721/Cargo.toml.hbs"),
            include_str!("../../templates/erc721/lib.rs.hbs"),
        ),
        "dao" => (
            include_str!("../../templates/dao/Cargo.toml.hbs"),
            include_str!("../../templates/dao/lib.rs.hbs"),
        ),
        _ => unreachable!(),
    };

    // Render Cargo.toml
    let cargo_toml_content = handlebars.render_template(cargo_toml_template, &template_data)?;
    let cargo_toml_path = Path::new(&args.name).join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;
    println!("  {} Created: Cargo.toml", "✓".green());

    // Render lib.rs
    let lib_rs_content = handlebars.render_template(lib_rs_template, &template_data)?;
    let lib_rs_path = Path::new(&args.name).join("lib.rs");
    fs::write(&lib_rs_path, lib_rs_content)?;
    println!("  {} Created: lib.rs", "✓".green());

    println!(
        "\n{} Contract project created successfully!",
        "✓".green().bold()
    );

    println!("\n{}", "Next steps:".bold());
    println!("  cd {}", args.name);
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
