use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub struct AnalyzeArgs {
    /// Contract file or directory to analyze
    #[arg(default_value = ".")]
    pub path: String,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    pub format: String,

    /// Show detailed analysis
    #[arg(short, long)]
    pub detailed: bool,

    /// Check for security issues
    #[arg(short, long)]
    pub security: bool,

    /// Analyze gas optimization opportunities
    #[arg(short, long)]
    pub gas: bool,

    /// Output file for results
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub summary: AnalysisSummary,
    pub files: Vec<FileAnalysis>,
    pub security_issues: Vec<SecurityIssue>,
    pub gas_optimizations: Vec<GasOptimization>,
    pub complexity_metrics: ComplexityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_functions: usize,
    pub security_issues_count: usize,
    pub gas_optimization_count: usize,
    pub average_complexity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub lines_of_code: usize,
    pub functions: Vec<FunctionInfo>,
    pub imports: Vec<String>,
    pub traits: Vec<String>,
    pub structs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub visibility: String,
    pub is_payable: bool,
    pub lines: usize,
    pub complexity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: String, // "high", "medium", "low"
    pub category: String,
    pub description: String,
    pub file: String,
    pub line: Option<usize>,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasOptimization {
    pub impact: String, // "high", "medium", "low"
    pub description: String,
    pub file: String,
    pub line: Option<usize>,
    pub suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: HashMap<String, u32>,
    pub cognitive_complexity: HashMap<String, u32>,
    pub maintainability_index: f64,
}

pub fn run(args: AnalyzeArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", args.path);
    }

    println!("{}", "Analyzing contracts...".cyan().bold());
    println!();

    let report = analyze_path(&path, &args)?;

    match args.format.as_str() {
        "json" => output_json(&report, args.output.as_deref())?,
        "text" | _ => output_text(&report, &args)?,
    }

    Ok(())
}

fn analyze_path(path: &Path, args: &AnalyzeArgs) -> Result<AnalysisReport> {
    let mut files = Vec::new();
    let mut security_issues = Vec::new();
    let mut gas_optimizations = Vec::new();

    if path.is_file() {
        if let Some(analysis) = analyze_file(path)? {
            if args.security {
                security_issues.extend(analyze_security(path, &analysis)?);
            }
            if args.gas {
                gas_optimizations.extend(analyze_gas(path, &analysis)?);
            }
            files.push(analysis);
        }
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(analysis) = analyze_file(&file_path)? {
                    if args.security {
                        security_issues.extend(analyze_security(&file_path, &analysis)?);
                    }
                    if args.gas {
                        gas_optimizations.extend(analyze_gas(&file_path, &analysis)?);
                    }
                    files.push(analysis);
                }
            }
        }
    }

    let total_lines: usize = files.iter().map(|f| f.lines_of_code).sum();
    let total_functions: usize = files.iter().map(|f| f.functions.len()).sum();
    let total_complexity: u32 = files
        .iter()
        .flat_map(|f| &f.functions)
        .map(|func| func.complexity)
        .sum();
    let average_complexity = if total_functions > 0 {
        total_complexity as f64 / total_functions as f64
    } else {
        0.0
    };

    let complexity_metrics = calculate_complexity_metrics(&files);

    Ok(AnalysisReport {
        summary: AnalysisSummary {
            total_files: files.len(),
            total_lines,
            total_functions,
            security_issues_count: security_issues.len(),
            gas_optimization_count: gas_optimizations.len(),
            average_complexity,
        },
        files,
        security_issues,
        gas_optimizations,
        complexity_metrics,
    })
}

fn analyze_file(path: &Path) -> Result<Option<FileAnalysis>> {
    let content = fs::read_to_string(path).context("Failed to read file")?;

    // Skip non-contract files
    if !content.contains("#[ink::contract]") && !content.contains("mod ") {
        return Ok(None);
    }

    let lines_of_code = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    let functions = extract_functions(&content);
    let imports = extract_imports(&content);
    let traits = extract_traits(&content);
    let structs = extract_structs(&content);

    Ok(Some(FileAnalysis {
        path: path.to_string_lossy().to_string(),
        lines_of_code,
        functions,
        imports,
        traits,
        structs,
    }))
}

fn extract_functions(content: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Look for function definitions
        if trimmed.starts_with("pub fn ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("pub(crate) fn ")
        {
            let name = extract_function_name(trimmed);
            let visibility = if trimmed.starts_with("pub ") {
                "public"
            } else {
                "private"
            }
            .to_string();

            // Check if payable (look for #[ink(payable)] in previous lines)
            let is_payable = i > 0 && lines[i - 1].contains("#[ink(payable)]");

            // Count lines in function body
            let func_lines = count_function_lines(&lines, i);

            // Calculate cyclomatic complexity
            let complexity = calculate_function_complexity(&lines, i);

            functions.push(FunctionInfo {
                name,
                visibility,
                is_payable,
                lines: func_lines,
                complexity,
            });
        }
    }

    functions
}

fn extract_function_name(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if *part == "fn" && i + 1 < parts.len() {
            let name = parts[i + 1];
            return name.split('(').next().unwrap_or(name).to_string();
        }
    }
    "unknown".to_string()
}

fn count_function_lines(lines: &[&str], start_idx: usize) -> usize {
    let mut count = 0;
    let mut brace_count = 0;
    let mut started = false;

    for line in lines.iter().skip(start_idx) {
        if line.contains('{') {
            brace_count += line.matches('{').count() as i32;
            started = true;
        }
        if line.contains('}') {
            brace_count -= line.matches('}').count() as i32;
        }

        if started {
            count += 1;
        }

        if started && brace_count == 0 {
            break;
        }
    }

    count
}

fn calculate_function_complexity(lines: &[&str], start_idx: usize) -> u32 {
    let mut complexity = 1; // Base complexity
    let mut in_function = false;
    let mut brace_count = 0;

    for line in lines.iter().skip(start_idx) {
        let trimmed = line.trim();

        if trimmed.contains('{') {
            brace_count += trimmed.matches('{').count() as i32;
            in_function = true;
        }
        if trimmed.contains('}') {
            brace_count -= trimmed.matches('}').count() as i32;
        }

        if in_function {
            // Count decision points
            if trimmed.contains("if ")
                || trimmed.contains("else if ")
                || trimmed.contains("match ")
                || trimmed.contains("while ")
                || trimmed.contains("for ")
                || trimmed.contains("loop ")
                || trimmed.contains("&&")
                || trimmed.contains("||")
            {
                complexity += 1;
            }

            // Match arms
            if trimmed.contains("=>") {
                complexity += 1;
            }
        }

        if in_function && brace_count == 0 {
            break;
        }
    }

    complexity
}

fn extract_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim().starts_with("use "))
        .map(|line| line.trim().to_string())
        .collect()
}

fn extract_traits(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim().starts_with("pub trait ") || line.trim().starts_with("trait "))
        .map(|line| {
            line.trim()
                .replace("pub trait ", "")
                .replace("trait ", "")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect()
}

fn extract_structs(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim().starts_with("pub struct ") || line.trim().starts_with("struct "))
        .map(|line| {
            line.trim()
                .replace("pub struct ", "")
                .replace("struct ", "")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect()
}

fn analyze_security(path: &Path, analysis: &FileAnalysis) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();
    let content = fs::read_to_string(path)?;

    // Check for common security issues

    // 1. Unchecked arithmetic operations
    if (content.contains(" + ") || content.contains(" - ") || content.contains(" * "))
        && !content.contains("checked_add")
            && !content.contains("checked_sub")
            && !content.contains("checked_mul")
        {
            issues.push(SecurityIssue {
                severity: "medium".to_string(),
                category: "Arithmetic".to_string(),
                description: "Potential integer overflow/underflow".to_string(),
                file: path.to_string_lossy().to_string(),
                line: None,
                recommendation: "Use checked arithmetic operations (checked_add, checked_sub, etc.)".to_string(),
            });
        }

    // 2. Missing access control on payable functions
    for func in &analysis.functions {
        if func.is_payable && func.visibility == "public"
            && !content.contains("only_owner") && !content.contains("require!") {
                issues.push(SecurityIssue {
                    severity: "high".to_string(),
                    category: "Access Control".to_string(),
                    description: format!("Payable function '{}' lacks access control", func.name),
                    file: path.to_string_lossy().to_string(),
                    line: None,
                    recommendation: "Add access control checks to prevent unauthorized calls"
                        .to_string(),
                });
            }
    }

    // 3. Unsafe unwrap usage
    if content.contains(".unwrap()") {
        issues.push(SecurityIssue {
            severity: "low".to_string(),
            category: "Error Handling".to_string(),
            description: "Use of unsafe unwrap() that could panic".to_string(),
            file: path.to_string_lossy().to_string(),
            line: None,
            recommendation: "Replace unwrap() with proper error handling using ? or expect()"
                .to_string(),
        });
    }

    // 4. Missing event emissions
    if content.contains("#[ink(message)]") && !content.contains("Self::env().emit_event") {
        issues.push(SecurityIssue {
            severity: "low".to_string(),
            category: "Transparency".to_string(),
            description: "State-changing functions should emit events".to_string(),
            file: path.to_string_lossy().to_string(),
            line: None,
            recommendation: "Emit events for important state changes for transparency".to_string(),
        });
    }

    Ok(issues)
}

fn analyze_gas(path: &Path, analysis: &FileAnalysis) -> Result<Vec<GasOptimization>> {
    let mut optimizations = Vec::new();
    let content = fs::read_to_string(path)?;

    // 1. String usage (expensive in storage)
    if content.contains("String") && content.contains("#[ink(storage)]") {
        optimizations.push(GasOptimization {
            impact: "high".to_string(),
            description: "String type in storage is expensive".to_string(),
            file: path.to_string_lossy().to_string(),
            line: None,
            suggestion: "Consider using Vec<u8> or bounded types for storage".to_string(),
        });
    }

    // 2. Large loop iterations
    if content.contains("for ") {
        optimizations.push(GasOptimization {
            impact: "medium".to_string(),
            description: "Loop iterations can be gas-intensive".to_string(),
            file: path.to_string_lossy().to_string(),
            line: None,
            suggestion: "Limit loop iterations or use pagination for large datasets".to_string(),
        });
    }

    // 3. Inefficient data structures
    if content.contains("Vec<") && content.contains("#[ink(storage)]") {
        optimizations.push(GasOptimization {
            impact: "medium".to_string(),
            description: "Vec in storage requires careful management".to_string(),
            file: path.to_string_lossy().to_string(),
            line: None,
            suggestion: "Consider using Mapping for key-value storage or BTreeMap for ordered data"
                .to_string(),
        });
    }

    // 4. High complexity functions
    for func in &analysis.functions {
        if func.complexity > 10 {
            optimizations.push(GasOptimization {
                impact: "medium".to_string(),
                description: format!(
                    "Function '{}' has high complexity ({})",
                    func.name, func.complexity
                ),
                file: path.to_string_lossy().to_string(),
                line: None,
                suggestion: "Consider breaking down into smaller functions to reduce gas costs"
                    .to_string(),
            });
        }
    }

    Ok(optimizations)
}

fn calculate_complexity_metrics(files: &[FileAnalysis]) -> ComplexityMetrics {
    let mut cyclomatic = HashMap::new();
    let mut cognitive = HashMap::new();

    for file in files {
        for func in &file.functions {
            cyclomatic.insert(func.name.clone(), func.complexity);
            // Cognitive complexity is similar but weights nested structures higher
            cognitive.insert(func.name.clone(), func.complexity);
        }
    }

    // Calculate maintainability index (simplified)
    let total_complexity: u32 = cyclomatic.values().sum();
    let total_functions = cyclomatic.len() as f64;
    let avg_complexity = if total_functions > 0.0 {
        total_complexity as f64 / total_functions
    } else {
        0.0
    };

    // Maintainability index (simplified formula)
    // 171 - 5.2 * ln(Halstead Volume) - 0.23 * (Cyclomatic Complexity) - 16.2 * ln(Lines of Code)
    // Simplified here to: 100 - (avg_complexity * 5)
    let maintainability = (100.0 - (avg_complexity * 5.0)).max(0.0).min(100.0);

    ComplexityMetrics {
        cyclomatic_complexity: cyclomatic,
        cognitive_complexity: cognitive,
        maintainability_index: maintainability,
    }
}

fn output_text(report: &AnalysisReport, args: &AnalyzeArgs) -> Result<()> {
    // Summary
    println!("{}", "=== Analysis Summary ===".green().bold());
    println!("Files analyzed:      {}", report.summary.total_files);
    println!("Total lines:         {}", report.summary.total_lines);
    println!("Total functions:     {}", report.summary.total_functions);
    println!(
        "Average complexity:  {:.2}",
        report.summary.average_complexity
    );
    println!(
        "Maintainability:     {:.1}/100",
        report.complexity_metrics.maintainability_index
    );
    println!();

    // Security issues
    if args.security && !report.security_issues.is_empty() {
        println!("{}", "=== Security Issues ===".red().bold());
        for issue in &report.security_issues {
            let severity_color = match issue.severity.as_str() {
                "high" => "red",
                "medium" => "yellow",
                "low" => "cyan",
                _ => "white",
            };

            println!(
                "  {} [{}] {}",
                "▸".bold(),
                issue.severity.color(severity_color).bold(),
                issue.description
            );
            println!("    Category: {}", issue.category);
            println!("    File: {}", issue.file);
            println!("    {}: {}", "Fix".green(), issue.recommendation);
            println!();
        }
    }

    // Gas optimizations
    if args.gas && !report.gas_optimizations.is_empty() {
        println!(
            "{}",
            "=== Gas Optimization Opportunities ===".yellow().bold()
        );
        for opt in &report.gas_optimizations {
            let impact_color = match opt.impact.as_str() {
                "high" => "red",
                "medium" => "yellow",
                "low" => "cyan",
                _ => "white",
            };

            println!(
                "  {} [{}] {}",
                "▸".bold(),
                opt.impact.color(impact_color).bold(),
                opt.description
            );
            println!("    File: {}", opt.file);
            println!("    {}: {}", "Suggestion".green(), opt.suggestion);
            println!();
        }
    }

    // Detailed file analysis
    if args.detailed {
        println!("{}", "=== Detailed Analysis ===".cyan().bold());
        for file in &report.files {
            println!("\n  {}", file.path.bold());
            println!("    Lines: {}", file.lines_of_code);
            println!("    Functions: {}", file.functions.len());
            println!("    Structs: {}", file.structs.len());
            println!("    Traits: {}", file.traits.len());

            if !file.functions.is_empty() {
                println!("\n    Functions:");
                for func in &file.functions {
                    let complexity_color = if func.complexity > 10 {
                        "red"
                    } else if func.complexity > 5 {
                        "yellow"
                    } else {
                        "green"
                    };

                    println!(
                        "      • {} ({}, {} lines, complexity: {})",
                        func.name.bold(),
                        func.visibility,
                        func.lines,
                        func.complexity.to_string().color(complexity_color)
                    );
                }
            }
        }
    }

    println!();
    println!("{}", "✓ Analysis complete!".green().bold());

    Ok(())
}

fn output_json(report: &AnalysisReport, output_file: Option<&str>) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;

    if let Some(file_path) = output_file {
        fs::write(file_path, json)?;
        println!(
            "{} {}",
            "✓".green(),
            format!("Report saved to {}", file_path)
        );
    } else {
        println!("{}", json);
    }

    Ok(())
}
