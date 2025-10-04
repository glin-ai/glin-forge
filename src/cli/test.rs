use clap::Parser;
use colored::Colorize;
use std::process::Command;

#[derive(Parser)]
pub struct TestArgs {
    /// Path to the contract project
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Run end-to-end tests
    #[arg(long)]
    pub e2e: bool,

    /// Test name filter
    #[arg(long)]
    pub test: Option<String>,

    /// Show output of successful tests
    #[arg(long)]
    pub nocapture: bool,
}

pub async fn execute(args: TestArgs) -> anyhow::Result<()> {
    println!("{}", "Running contract tests...".cyan().bold());

    let test_type = if args.e2e { "E2E" } else { "Unit" };
    println!("  {} {} tests", "→".cyan(), test_type);

    if let Some(filter) = &args.test {
        println!("  {} Filtering by: {}", "→".cyan(), filter);
    }

    // Check if cargo-contract is installed
    let cargo_contract_check = Command::new("cargo")
        .arg("contract")
        .arg("--version")
        .output();

    if cargo_contract_check.is_err() {
        anyhow::bail!(
            "cargo-contract not found. Install it with: {}",
            "cargo install cargo-contract --force".yellow()
        );
    }

    println!();

    // Run tests
    let mut cmd = Command::new("cargo");

    if args.e2e {
        cmd.arg("test")
            .arg("--features")
            .arg("e2e-tests");
    } else {
        cmd.arg("test");
    }

    if let Some(filter) = &args.test {
        cmd.arg(filter);
    }

    if args.nocapture {
        cmd.arg("--")
            .arg("--nocapture");
    }

    cmd.current_dir(&args.path);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
        eprintln!("{}", stderr);
        anyhow::bail!("Tests failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    println!(
        "\n{} All tests passed!",
        "✓".green().bold()
    );

    Ok(())
}
