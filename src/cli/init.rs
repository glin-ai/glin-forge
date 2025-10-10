use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
pub struct InitArgs {
    /// Project path (default: current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Skip interactive prompts and use defaults
    #[arg(long)]
    pub yes: bool,

    /// Template to use (erc20, erc721, dao, flipper, basic)
    #[arg(short, long)]
    pub template: Option<String>,

    /// Project type (basic, fullstack, library)
    #[arg(long)]
    pub project_type: Option<String>,

    /// Frontend framework (none, react, nextjs, vue)
    #[arg(long)]
    pub frontend: Option<String>,
}

#[derive(Debug, Clone)]
enum ProjectType {
    Basic,
    Fullstack,
    Library,
}

impl ProjectType {
    fn as_str(&self) -> &str {
        match self {
            ProjectType::Basic => "basic",
            ProjectType::Fullstack => "fullstack",
            ProjectType::Library => "library",
        }
    }
}

#[derive(Debug, Clone)]
enum Frontend {
    None,
    React,
    NextJs,
    Vue,
}

impl Frontend {
    fn as_str(&self) -> &str {
        match self {
            Frontend::None => "none",
            Frontend::React => "react",
            Frontend::NextJs => "nextjs",
            Frontend::Vue => "vue",
        }
    }
}

pub async fn execute(args: InitArgs) -> anyhow::Result<()> {
    println!("{}", "🚀 Initialize new glin-forge project".cyan().bold());
    println!();

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
            anyhow::bail!(
                "Directory is not empty. Use 'glin-forge new <name>' to create a new project."
            );
        }
    } else {
        fs::create_dir_all(path)?;
    }

    // Get project name from directory
    let default_project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my_contract")
        .to_string();

    // Interactive prompts or use defaults
    let (project_name, project_type, template, frontend, init_git, install_deps) = if args.yes {
        // Use defaults
        (
            default_project_name,
            ProjectType::Basic,
            args.template.unwrap_or_else(|| "erc20".to_string()),
            Frontend::None,
            false,
            false,
        )
    } else {
        // Interactive prompts
        interactive_setup(
            &default_project_name,
            args.project_type,
            args.template,
            args.frontend,
        )?
    };

    println!();
    println!("{}", "📦 Project Configuration".bold());
    println!("  {} {}", "Name:".cyan(), project_name);
    println!("  {} {}", "Type:".cyan(), project_type.as_str());
    println!("  {} {}", "Template:".cyan(), template);
    println!("  {} {}", "Frontend:".cyan(), frontend.as_str());
    println!();

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

    // Create contract files
    println!("{}", "📝 Creating files...".bold());
    create_contract_files(path, &template, &handlebars, &template_data)?;

    // Create config file
    create_config_file(path, &project_type, &frontend)?;

    // Create frontend if needed
    if !matches!(frontend, Frontend::None) {
        create_frontend(path, &frontend, &project_name)?;
    }

    // Create .gitignore
    create_gitignore(path, &frontend)?;

    // Initialize git if requested
    if init_git {
        println!();
        println!("{}", "🔧 Initializing git...".bold());
        init_git_repo(path)?;
    }

    // Install dependencies if requested
    if install_deps {
        println!();
        println!("{}", "📥 Installing dependencies...".bold());
        install_dependencies(path, &frontend)?;
    }

    println!();
    println!(
        "{} {}",
        "✅".green().bold(),
        "Project initialized successfully!".green().bold()
    );

    println!();
    println!("{}", "📚 Next steps:".bold());
    if args.path != "." {
        println!("  cd {}", args.path);
    }
    println!("  glin-forge build");
    println!("  glin-forge deploy --network testnet");
    if !matches!(frontend, Frontend::None) {
        println!("  cd frontend && npm run dev");
    }

    Ok(())
}

fn interactive_setup(
    default_name: &str,
    project_type_arg: Option<String>,
    template_arg: Option<String>,
    frontend_arg: Option<String>,
) -> anyhow::Result<(String, ProjectType, String, Frontend, bool, bool)> {
    let theme = ColorfulTheme::default();

    // Project name
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("Project name")
        .default(default_name.to_string())
        .interact_text()?;

    // Project type
    let project_type = if let Some(pt) = project_type_arg {
        match pt.as_str() {
            "basic" => ProjectType::Basic,
            "fullstack" => ProjectType::Fullstack,
            "library" => ProjectType::Library,
            _ => ProjectType::Basic,
        }
    } else {
        let types = vec![
            "Basic contract project",
            "Full-stack dApp (contract + frontend)",
            "Contract library (multiple contracts)",
        ];
        let selection = Select::with_theme(&theme)
            .with_prompt("What type of project?")
            .items(&types)
            .default(0)
            .interact()?;

        match selection {
            0 => ProjectType::Basic,
            1 => ProjectType::Fullstack,
            2 => ProjectType::Library,
            _ => ProjectType::Basic,
        }
    };

    // Template
    let template = if let Some(t) = template_arg {
        t
    } else {
        let templates = vec![
            "erc20 - ERC20 token contract",
            "erc721 - NFT contract",
            "flipper - Simple boolean flipper",
            "dao - DAO governance contract",
            "basic - Empty contract",
        ];
        let selection = Select::with_theme(&theme)
            .with_prompt("Choose a contract template")
            .items(&templates)
            .default(0)
            .interact()?;

        match selection {
            0 => "erc20",
            1 => "erc721",
            2 => "flipper",
            3 => "dao",
            4 => "basic",
            _ => "erc20",
        }
        .to_string()
    };

    // Frontend (only for fullstack)
    let frontend = if matches!(project_type, ProjectType::Fullstack) {
        if let Some(f) = frontend_arg {
            match f.as_str() {
                "react" => Frontend::React,
                "nextjs" => Frontend::NextJs,
                "vue" => Frontend::Vue,
                _ => Frontend::None,
            }
        } else {
            let frameworks = vec![
                "React - React + TypeScript",
                "Next.js - React framework with SSR",
                "Vue - Vue 3 + TypeScript",
                "None - Contract only",
            ];
            let selection = Select::with_theme(&theme)
                .with_prompt("Choose a frontend framework")
                .items(&frameworks)
                .default(0)
                .interact()?;

            match selection {
                0 => Frontend::React,
                1 => Frontend::NextJs,
                2 => Frontend::Vue,
                3 => Frontend::None,
                _ => Frontend::None,
            }
        }
    } else {
        Frontend::None
    };

    // Git initialization
    let init_git = Confirm::with_theme(&theme)
        .with_prompt("Initialize git repository?")
        .default(true)
        .interact()?;

    // Install dependencies
    let install_deps = if !matches!(frontend, Frontend::None) {
        Confirm::with_theme(&theme)
            .with_prompt("Install dependencies?")
            .default(true)
            .interact()?
    } else {
        false
    };

    Ok((
        project_name,
        project_type,
        template,
        frontend,
        init_git,
        install_deps,
    ))
}

fn create_contract_files(
    path: &Path,
    template: &str,
    handlebars: &Handlebars,
    template_data: &serde_json::Value,
) -> anyhow::Result<()> {
    // Get template files based on template name
    let (cargo_toml_template, lib_rs_template) = match template {
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
        _ => {
            // Default to erc20 for unknown templates
            (
                include_str!("../../templates/erc20/Cargo.toml.hbs"),
                include_str!("../../templates/erc20/lib.rs.hbs"),
            )
        }
    };

    // Render Cargo.toml
    let cargo_toml_content = handlebars.render_template(cargo_toml_template, template_data)?;
    let cargo_toml_path = path.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;
    println!("  {} Created: Cargo.toml", "✓".green());

    // Render lib.rs
    let lib_rs_content = handlebars.render_template(lib_rs_template, template_data)?;
    let lib_rs_path = path.join("lib.rs");
    fs::write(&lib_rs_path, lib_rs_content)?;
    println!("  {} Created: lib.rs", "✓".green());

    Ok(())
}

fn create_config_file(
    path: &Path,
    project_type: &ProjectType,
    frontend: &Frontend,
) -> anyhow::Result<()> {
    let config_content = match (project_type, frontend) {
        (ProjectType::Fullstack, Frontend::None) => {
            include_str!("../../templates/config/glinforge.config.ts")
        }
        (ProjectType::Fullstack, _) => {
            include_str!("../../templates/config/glinforge.config.fullstack.ts")
        }
        _ => include_str!("../../templates/config/glinforge.config.minimal.ts"),
    };

    let config_path = path.join("glinforge.config.ts");
    fs::write(&config_path, config_content)?;
    println!("  {} Created: glinforge.config.ts", "✓".green());

    Ok(())
}

fn create_frontend(path: &Path, frontend: &Frontend, project_name: &str) -> anyhow::Result<()> {
    println!();
    println!("{}", "🎨 Creating frontend...".bold());

    let frontend_path = path.join("frontend");
    fs::create_dir_all(&frontend_path)?;

    match frontend {
        Frontend::React => {
            // Create basic React app structure
            create_react_app(&frontend_path, project_name)?;
        }
        Frontend::NextJs => {
            // Create basic Next.js app structure
            create_nextjs_app(&frontend_path, project_name)?;
        }
        Frontend::Vue => {
            // Create basic Vue app structure
            create_vue_app(&frontend_path, project_name)?;
        }
        Frontend::None => {}
    }

    Ok(())
}

fn create_react_app(path: &Path, project_name: &str) -> anyhow::Result<()> {
    // Create package.json
    let package_json = json!({
        "name": format!("{}-frontend", project_name),
        "version": "0.1.0",
        "private": true,
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0",
            "@glin-forge/sdk": "^0.1.0",
            "typescript": "^5.0.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview",
            "typecheck": "tsc --noEmit"
        },
        "devDependencies": {
            "@types/react": "^18.2.0",
            "@types/react-dom": "^18.2.0",
            "@vitejs/plugin-react": "^4.0.0",
            "vite": "^5.0.0"
        }
    });

    fs::write(
        path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?,
    )?;
    println!("  {} Created: frontend/package.json", "✓".green());

    // Create src directory
    let src_path = path.join("src");
    fs::create_dir_all(&src_path)?;

    // Setup handlebars for templates
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);

    let contract_name = project_name.replace("-", "_");
    let template_data = json!({
        "project_name": project_name,
        "contract_name": contract_name,
    });

    // Create App.tsx
    let app_template = include_str!("../../templates/frontend/react/App.tsx.hbs");
    let app_content = handlebars.render_template(app_template, &template_data)?;
    fs::write(src_path.join("App.tsx"), app_content)?;
    println!("  {} Created: frontend/src/App.tsx", "✓".green());

    // Create main.tsx
    let main_template = include_str!("../../templates/frontend/react/main.tsx.hbs");
    let main_content = handlebars.render_template(main_template, &template_data)?;
    fs::write(src_path.join("main.tsx"), main_content)?;
    println!("  {} Created: frontend/src/main.tsx", "✓".green());

    // Create App.css
    let app_css = include_str!("../../templates/frontend/react/App.css.hbs");
    fs::write(src_path.join("App.css"), app_css)?;
    println!("  {} Created: frontend/src/App.css", "✓".green());

    // Create index.css
    let index_css = include_str!("../../templates/frontend/react/index.css.hbs");
    fs::write(src_path.join("index.css"), index_css)?;
    println!("  {} Created: frontend/src/index.css", "✓".green());

    // Create config.ts
    let config_template = include_str!("../../templates/frontend/react/config.ts.hbs");
    let config_content = handlebars.render_template(config_template, &template_data)?;
    fs::write(src_path.join("config.ts"), config_content)?;
    println!("  {} Created: frontend/src/config.ts", "✓".green());

    // Create index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>GLIN dApp</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"#;
    fs::write(path.join("index.html"), index_html)?;
    println!("  {} Created: frontend/index.html", "✓".green());

    // Create vite.config.ts
    let vite_config = include_str!("../../templates/frontend/react/vite.config.ts.hbs");
    fs::write(path.join("vite.config.ts"), vite_config)?;
    println!("  {} Created: frontend/vite.config.ts", "✓".green());

    // Create tsconfig.json
    let tsconfig = include_str!("../../templates/frontend/react/tsconfig.json.hbs");
    fs::write(path.join("tsconfig.json"), tsconfig)?;
    println!("  {} Created: frontend/tsconfig.json", "✓".green());

    // Create tsconfig.node.json
    let tsconfig_node = include_str!("../../templates/frontend/react/tsconfig.node.json.hbs");
    fs::write(path.join("tsconfig.node.json"), tsconfig_node)?;
    println!("  {} Created: frontend/tsconfig.node.json", "✓".green());

    Ok(())
}

fn create_nextjs_app(path: &Path, project_name: &str) -> anyhow::Result<()> {
    // Create package.json
    let package_json = json!({
        "name": format!("{}-frontend", project_name),
        "version": "0.1.0",
        "private": true,
        "scripts": {
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "typecheck": "tsc --noEmit"
        },
        "dependencies": {
            "next": "^14.0.0",
            "react": "^18.2.0",
            "react-dom": "^18.2.0",
            "@glin-forge/sdk": "^0.1.0"
        },
        "devDependencies": {
            "@types/node": "^20.0.0",
            "@types/react": "^18.2.0",
            "typescript": "^5.0.0"
        }
    });

    fs::write(
        path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?,
    )?;
    println!("  {} Created: frontend/package.json", "✓".green());

    // Create app directory for Next.js 13+ app router
    let app_path = path.join("app");
    fs::create_dir_all(&app_path)?;

    // Create app/page.tsx
    let page_tsx = format!(
        r#"'use client';

import {{ useState, useEffect }} from 'react';
import styles from './page.module.css';

export default function Home() {{
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {{
    const rpcPort = process.env.NEXT_PUBLIC_GLIN_FORGE_RPC_PORT;
    setIsConnected(!!rpcPort);
  }}, []);

  return (
    <main className={{styles.main}}>
      <h1>{}</h1>
      <p>GLIN Network dApp with Next.js</p>

      {{isConnected ? (
        <div className={{styles.connected}}>
          Connected to glin-forge
        </div>
      ) : (
        <div className={{styles.disconnected}}>
          Not connected - Run with: glin-forge run scripts/dev.ts
        </div>
      )}}

      <div className={{styles.card}}>
        <h2>Quick Start</h2>
        <ol>
          <li>Run <code>glin-forge build</code></li>
          <li>Run <code>glin-forge deploy</code></li>
          <li>Update contract address in <code>config.ts</code></li>
          <li>Start building your dApp!</li>
        </ol>
      </div>
    </main>
  );
}}
"#,
        project_name
    );

    fs::write(app_path.join("page.tsx"), page_tsx)?;
    println!("  {} Created: frontend/app/page.tsx", "✓".green());

    // Create app/layout.tsx
    let layout_tsx = format!(
        r#"import type {{ Metadata }} from 'next';
import './globals.css';

export const metadata: Metadata = {{
  title: '{}',
  description: 'GLIN Network dApp',
}};

export default function RootLayout({{
  children,
}}: {{
  children: React.ReactNode;
}}) {{
  return (
    <html lang="en">
      <body>{{children}}</body>
    </html>
  );
}}
"#,
        project_name
    );

    fs::write(app_path.join("layout.tsx"), layout_tsx)?;
    println!("  {} Created: frontend/app/layout.tsx", "✓".green());

    // Create app/globals.css
    let globals_css = r#"* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  background-color: #f8f9fa;
}
"#;
    fs::write(app_path.join("globals.css"), globals_css)?;
    println!("  {} Created: frontend/app/globals.css", "✓".green());

    // Create app/page.module.css
    let page_css = r#".main {
  min-height: 100vh;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.connected {
  background-color: #d4edda;
  color: #155724;
  padding: 1rem;
  border-radius: 8px;
  margin: 1rem 0;
}

.disconnected {
  background-color: #f8d7da;
  color: #721c24;
  padding: 1rem;
  border-radius: 8px;
  margin: 1rem 0;
}

.card {
  background: white;
  border-radius: 12px;
  padding: 2rem;
  margin: 2rem 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
"#;
    fs::write(app_path.join("page.module.css"), page_css)?;
    println!("  {} Created: frontend/app/page.module.css", "✓".green());

    // Create tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
"#;
    fs::write(path.join("tsconfig.json"), tsconfig)?;
    println!("  {} Created: frontend/tsconfig.json", "✓".green());

    // Create next.config.js
    let next_config = r#"/** @type {import('next').NextConfig} */
const nextConfig = {
  env: {
    NEXT_PUBLIC_GLIN_FORGE_RPC_PORT: process.env.GLIN_FORGE_RPC_PORT,
  },
};

module.exports = nextConfig;
"#;
    fs::write(path.join("next.config.js"), next_config)?;
    println!("  {} Created: frontend/next.config.js", "✓".green());

    Ok(())
}

fn create_vue_app(path: &Path, project_name: &str) -> anyhow::Result<()> {
    // Create package.json
    let package_json = json!({
        "name": format!("{}-frontend", project_name),
        "version": "0.1.0",
        "private": true,
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview",
            "typecheck": "vue-tsc --noEmit"
        },
        "dependencies": {
            "vue": "^3.3.0",
            "@glin-forge/sdk": "^0.1.0"
        },
        "devDependencies": {
            "@vitejs/plugin-vue": "^4.0.0",
            "typescript": "^5.0.0",
            "vite": "^5.0.0",
            "vue-tsc": "^1.8.0"
        }
    });

    fs::write(
        path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?,
    )?;
    println!("  {} Created: frontend/package.json", "✓".green());

    // Create src directory
    let src_path = path.join("src");
    fs::create_dir_all(&src_path)?;

    // Create App.vue
    let app_vue = format!(
        r#"<script setup lang="ts">
import {{ ref, onMounted }} from 'vue';

const isConnected = ref(false);
const contractAddress = ref('');

onMounted(() => {{
  const rpcPort = import.meta.env.VITE_GLIN_FORGE_RPC_PORT;
  isConnected.value = !!rpcPort;
}});

const handleDeploy = async () => {{
  // TODO: Implement contract deployment
  console.log('Deploy contract here');
}};
</script>

<template>
  <div class="app">
    <header class="header">
      <h1>{}</h1>
      <p>GLIN Network dApp with Vue</p>
    </header>

    <main class="main">
      <div v-if="isConnected" class="status connected">
        Connected to glin-forge
      </div>
      <div v-else class="status disconnected">
        Not connected - Run with: glin-forge run scripts/dev.ts
      </div>

      <div class="card">
        <h2>Contract Deployment</h2>
        <div v-if="contractAddress" class="contract-info">
          <p>Contract deployed at:</p>
          <code>{{{{ contractAddress }}}}</code>
        </div>
        <button v-else @click="handleDeploy" :disabled="!isConnected" class="primary-button">
          Deploy Contract
        </button>
      </div>

      <div class="card">
        <h2>Quick Start</h2>
        <ol>
          <li>Run <code>glin-forge build</code></li>
          <li>Run <code>glin-forge deploy</code></li>
          <li>Update contract address in <code>src/config.ts</code></li>
          <li>Start building your dApp!</li>
        </ol>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app {{
  min-height: 100vh;
}}

.header {{
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 2rem;
  color: white;
  text-align: center;
}}

.main {{
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}}

.status {{
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 2rem;
}}

.connected {{
  background-color: #d4edda;
  color: #155724;
}}

.disconnected {{
  background-color: #f8d7da;
  color: #721c24;
}}

.card {{
  background: white;
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}}

.primary-button {{
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.75rem 2rem;
  font-size: 1rem;
  border-radius: 8px;
  cursor: pointer;
}}

.primary-button:disabled {{
  opacity: 0.5;
  cursor: not-allowed;
}}
</style>
"#,
        project_name
    );

    fs::write(src_path.join("App.vue"), app_vue)?;
    println!("  {} Created: frontend/src/App.vue", "✓".green());

    // Create main.ts
    let main_ts = r#"import { createApp } from 'vue';
import App from './App.vue';
import './style.css';

createApp(App).mount('#app');
"#;
    fs::write(src_path.join("main.ts"), main_ts)?;
    println!("  {} Created: frontend/src/main.ts", "✓".green());

    // Create style.css
    let style_css = r#"* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  background-color: #f8f9fa;
}

#app {
  min-height: 100vh;
}
"#;
    fs::write(src_path.join("style.css"), style_css)?;
    println!("  {} Created: frontend/src/style.css", "✓".green());

    // Create index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>GLIN dApp</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>"#;
    fs::write(path.join("index.html"), index_html)?;
    println!("  {} Created: frontend/index.html", "✓".green());

    // Create vite.config.ts
    let vite_config = r#"import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 3000,
  },
  define: {
    'import.meta.env.VITE_GLIN_FORGE_RPC_PORT': JSON.stringify(process.env.GLIN_FORGE_RPC_PORT),
  },
});
"#;
    fs::write(path.join("vite.config.ts"), vite_config)?;
    println!("  {} Created: frontend/vite.config.ts", "✓".green());

    // Create tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,

    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",

    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
"#;
    fs::write(path.join("tsconfig.json"), tsconfig)?;
    println!("  {} Created: frontend/tsconfig.json", "✓".green());

    // Create tsconfig.node.json
    let tsconfig_node = r#"{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
"#;
    fs::write(path.join("tsconfig.node.json"), tsconfig_node)?;
    println!("  {} Created: frontend/tsconfig.node.json", "✓".green());

    Ok(())
}

fn create_gitignore(path: &Path, frontend: &Frontend) -> anyhow::Result<()> {
    let mut gitignore_content = String::from(
        r#"# Rust
target/
Cargo.lock
**/*.rs.bk
*.pdb

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Cache
.cache/
"#,
    );

    if !matches!(frontend, Frontend::None) {
        gitignore_content.push_str(
            r#"
# Frontend
frontend/node_modules/
frontend/.next/
frontend/dist/
frontend/build/
"#,
        );
    }

    let gitignore_path = path.join(".gitignore");
    fs::write(&gitignore_path, gitignore_content)?;
    println!("  {} Created: .gitignore", "✓".green());

    Ok(())
}

fn init_git_repo(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("  {} Git repository initialized", "✓".green());
            Ok(())
        }
        _ => {
            println!("  {} Git not available, skipping", "⚠".yellow());
            Ok(())
        }
    }
}

fn install_dependencies(path: &Path, frontend: &Frontend) -> anyhow::Result<()> {
    if matches!(frontend, Frontend::None) {
        return Ok(());
    }

    let frontend_path = path.join("frontend");

    println!("  Installing frontend dependencies...");

    let output = Command::new("npm")
        .args(["install"])
        .current_dir(&frontend_path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("  {} Dependencies installed", "✓".green());
            Ok(())
        }
        _ => {
            println!("  {} npm not available, skipping", "⚠".yellow());
            println!("    Run 'cd frontend && npm install' manually");
            Ok(())
        }
    }
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
