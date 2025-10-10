use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Args)]
pub struct ConsoleArgs {
    /// Network to connect to
    #[arg(short, long, default_value = "local")]
    pub network: String,

    /// Contract address to interact with
    #[arg(short, long)]
    pub contract: Option<String>,

    /// Path to contract artifacts
    #[arg(long, default_value = "./artifacts")]
    pub artifacts_path: String,

    /// Show welcome banner
    #[arg(long, default_value = "true")]
    pub banner: bool,
}

pub async fn execute(args: ConsoleArgs) -> Result<()> {
    if args.banner {
        print_banner();
    }

    // Check if Node.js is available
    check_nodejs()?;

    // Load network configuration
    let network_config = load_network_config(&args.network)?;

    // Create REPL script
    let repl_script = create_repl_script(&args, &network_config)?;

    // Write temporary REPL file
    let temp_file = std::env::temp_dir().join("glin-forge-console.js");
    fs::write(&temp_file, repl_script).context("Failed to write REPL script")?;

    println!(
        "{}",
        format!("Connecting to network: {}", args.network)
            .cyan()
            .bold()
    );
    println!(
        "{}",
        format!("RPC endpoint: {}", network_config.rpc).dimmed()
    );

    if let Some(contract_addr) = &args.contract {
        println!(
            "{}",
            format!("Contract address: {}", contract_addr).dimmed()
        );
    }

    println!();
    println!("{}", "Type '.help' for more information".dimmed());
    println!();

    // Run Node.js REPL
    let status = Command::new("node")
        .arg(&temp_file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to start Node.js REPL")?;

    // Cleanup
    let _ = fs::remove_file(temp_file);

    if !status.success() {
        anyhow::bail!("REPL exited with error");
    }

    Ok(())
}

fn check_nodejs() -> Result<()> {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .context("Node.js not found. Please install Node.js to use the console.")?;

    if !output.status.success() {
        anyhow::bail!("Node.js is not working properly");
    }

    Ok(())
}

#[derive(Debug)]
struct NetworkConfig {
    rpc: String,
}

fn load_network_config(network: &str) -> Result<NetworkConfig> {
    // Try to load from glinforge.config.ts/js
    // For now, use defaults
    let rpc = match network {
        "local" => "ws://localhost:9944",
        "testnet" => "wss://testnet-rpc.glin.network",
        "mainnet" => "wss://rpc.glin.network",
        custom if custom.starts_with("ws://") || custom.starts_with("wss://") => custom,
        _ => {
            anyhow::bail!("Unknown network: {}. Use 'local', 'testnet', 'mainnet', or provide a WebSocket URL", network);
        }
    };

    Ok(NetworkConfig {
        rpc: rpc.to_string(),
    })
}

fn create_repl_script(args: &ConsoleArgs, network_config: &NetworkConfig) -> Result<String> {
    let artifacts_path = PathBuf::from(&args.artifacts_path);
    let artifacts_path_str = artifacts_path
        .canonicalize()
        .unwrap_or(artifacts_path)
        .to_string_lossy()
        .to_string();

    let script = format!(
        r#"
const repl = require('repl');
const {{ ApiPromise, WsProvider }} = require('@polkadot/api');
const {{ Keyring }} = require('@polkadot/keyring');
const {{ cryptoWaitReady }} = require('@polkadot/util-crypto');
const fs = require('fs');
const path = require('path');

// ANSI colors
const colors = {{
  reset: '\x1b[0m',
  cyan: '\x1b[36m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  dim: '\x1b[2m'
}};

async function initConsole() {{
  try {{
    // Initialize crypto
    await cryptoWaitReady();

    // Connect to network
    const provider = new WsProvider('{}');
    const api = await ApiPromise.create({{ provider }});

    // Initialize keyring
    const keyring = new Keyring({{ type: 'sr25519' }});

    // Load common accounts
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');
    const dave = keyring.addFromUri('//Dave');
    const eve = keyring.addFromUri('//Eve');

    // Load artifacts
    let artifacts = {{}};
    const artifactsPath = '{}';

    if (fs.existsSync(artifactsPath)) {{
      const files = fs.readdirSync(artifactsPath);
      for (const file of files) {{
        if (file.endsWith('.json')) {{
          const contractName = file.replace('.json', '');
          const artifactPath = path.join(artifactsPath, file);
          try {{
            artifacts[contractName] = JSON.parse(fs.readFileSync(artifactPath, 'utf-8'));
          }} catch (e) {{
            // Ignore invalid JSON
          }}
        }}
      }}
    }}

    // Helper functions
    const helpers = {{
      // Get balance of an account
      async getBalance(address) {{
        const {{ data: {{ free }} }} = await api.query.system.account(address);
        return free;
      }},

      // Format balance to human-readable string
      formatBalance(balance) {{
        return api.registry.createType('Balance', balance).toHuman();
      }},

      // Get current block number
      async getBlockNumber() {{
        const header = await api.rpc.chain.getHeader();
        return header.number.toNumber();
      }},

      // Wait for next block
      async nextBlock() {{
        return new Promise((resolve) => {{
          const unsub = api.rpc.chain.subscribeNewHeads((header) => {{
            unsub.then(u => u());
            resolve(header.number.toNumber());
          }});
        }});
      }},

      // Transfer tokens
      async transfer(from, to, amount) {{
        return new Promise((resolve, reject) => {{
          api.tx.balances.transfer(to, amount)
            .signAndSend(from, ({{ status, events }}) => {{
              if (status.isInBlock) {{
                console.log(`${{colors.green}}✓ Transaction included in block${{colors.reset}}`);
                resolve({{ status, events }});
              }}
            }})
            .catch(reject);
        }});
      }},

      // List all available contracts from artifacts
      listContracts() {{
        console.log(`${{colors.cyan}}Available contracts:${{colors.reset}}`);
        Object.keys(artifacts).forEach(name => {{
          console.log(`  • ${{name}}`);
        }});
      }},

      // Get contract ABI
      getAbi(contractName) {{
        return artifacts[contractName]?.abi;
      }},

      // Show help
      help() {{
        console.log(`${{colors.cyan}}glin-forge Console Commands:${{colors.reset}}`);
        console.log('');
        console.log('  ${{colors.yellow}}Available globals:${{colors.reset}}');
        console.log('    api          - Polkadot.js API instance');
        console.log('    keyring      - Keyring instance');
        console.log('    alice, bob   - Test accounts');
        console.log('    charlie, dave, eve - More test accounts');
        console.log('    artifacts    - Contract artifacts');
        console.log('');
        console.log('  ${{colors.yellow}}Helper functions:${{colors.reset}}');
        console.log('    getBalance(address)       - Get account balance');
        console.log('    formatBalance(balance)    - Format balance to string');
        console.log('    getBlockNumber()          - Get current block number');
        console.log('    nextBlock()               - Wait for next block');
        console.log('    transfer(from, to, amt)   - Transfer tokens');
        console.log('    listContracts()           - List available contracts');
        console.log('    getAbi(name)              - Get contract ABI');
        console.log('    help()                    - Show this help');
        console.log('');
        console.log('  ${{colors.yellow}}REPL commands:${{colors.reset}}');
        console.log('    .break       - Exit multiline mode');
        console.log('    .clear       - Clear REPL context');
        console.log('    .exit        - Exit REPL');
        console.log('    .help        - Show REPL help');
        console.log('    .save        - Save session to file');
        console.log('    .load        - Load session from file');
        console.log('');
      }}
    }};

    // Create REPL
    const replServer = repl.start({{
      prompt: `${{colors.cyan}}glin-forge>${{colors.reset}} `,
      useColors: true,
      ignoreUndefined: true,
    }});

    // Add globals to REPL context
    replServer.context.api = api;
    replServer.context.keyring = keyring;
    replServer.context.alice = alice;
    replServer.context.bob = bob;
    replServer.context.charlie = charlie;
    replServer.context.dave = dave;
    replServer.context.eve = eve;
    replServer.context.artifacts = artifacts;

    // Add helper functions
    Object.assign(replServer.context, helpers);

    // Handle REPL exit
    replServer.on('exit', async () => {{
      console.log('');
      console.log(`${{colors.green}}Disconnecting...${{colors.reset}}`);
      await api.disconnect();
      process.exit(0);
    }});

    // Connection success message
    const chainInfo = await api.rpc.system.chain();
    const nodeVersion = await api.rpc.system.version();

    console.log(`${{colors.green}}✓ Connected to ${{chainInfo}}${{colors.reset}}`);
    console.log(`${{colors.dim}}  Node version: ${{nodeVersion}}${{colors.reset}}`);

    if (Object.keys(artifacts).length > 0) {{
      console.log(`${{colors.dim}}  Loaded ${{Object.keys(artifacts).length}} contract(s)${{colors.reset}}`);
    }}

    console.log('');

  }} catch (error) {{
    console.error(`${{colors.yellow}}Error: ${{error.message}}${{colors.reset}}`);
    process.exit(1);
  }}
}}

// Start console
initConsole();
"#,
        network_config.rpc, artifacts_path_str
    );

    Ok(script)
}

fn print_banner() {
    let banner = r#"
   ______ _     _         ______
  / ____|| |   (_)       |  ____|
 | |  __ | |    _  _ __  | |__ ___   _ __  __ _   ___
 | | |_ || |   | || '_ \ |  __/ _ \ | '__|/ _` | / _ \
 | |__| || |___| || | | || | | (_) || |  | (_| ||  __/
  \_____||_____||_||_| |_||_|  \___/ |_|   \__, | \___|
                                            __/ |
                                           |___/
    "#;

    println!("{}", banner.cyan().bold());
    println!("{}", "Interactive Console for Smart Contracts".yellow());
    println!();
}
