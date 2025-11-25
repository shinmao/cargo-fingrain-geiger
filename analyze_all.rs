#!/usr/bin/env rust-script
//! Analyze all cargo projects in /home/crates/source/ for unsafe usage
//!
//! ```cargo
//! [dependencies]
//! walkdir = "2.5"
//! serde_json = "1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! ```

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug, Default, Clone)]
struct UnsafeStats {
    functions_unsafe: u64,
    functions_total: u64,
    expressions_unsafe: u64,
    expressions_total: u64,
    impls_unsafe: u64,
    impls_total: u64,
    traits_unsafe: u64,
    traits_total: u64,
    methods_unsafe: u64,
    methods_total: u64,
    ptr_derefs_unsafe: u64,
    ptr_derefs_total: u64,
    unsafe_calls_unsafe: u64,
    unsafe_calls_total: u64,
    forbids_unsafe: bool,
}

impl UnsafeStats {
    fn parse_from_line(line: &str) -> Option<Self> {
        // Parse line like: "1/1    2/2    0/0    0/0    0/0    2/2    0/0    ! crate_name"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }

        let parse_fraction = |s: &str| -> (u64, u64) {
            let nums: Vec<&str> = s.split('/').collect();
            if nums.len() == 2 {
                let unsafe_count = nums[0].parse::<u64>().unwrap_or(0);
                let total = nums[1].parse::<u64>().unwrap_or(0);
                (unsafe_count, total)
            } else {
                (0, 0)
            }
        };

        let (functions_unsafe, functions_total) = parse_fraction(parts[0]);
        let (expressions_unsafe, expressions_total) = parse_fraction(parts[1]);
        let (impls_unsafe, impls_total) = parse_fraction(parts[2]);
        let (traits_unsafe, traits_total) = parse_fraction(parts[3]);
        let (methods_unsafe, methods_total) = parse_fraction(parts[4]);
        let (ptr_derefs_unsafe, ptr_derefs_total) = parse_fraction(parts[5]);
        let (unsafe_calls_unsafe, unsafe_calls_total) = parse_fraction(parts[6]);

        // Check for forbids unsafe marker
        let forbids_unsafe = parts.get(7).map(|&s| s == ":)").unwrap_or(false);

        Some(UnsafeStats {
            functions_unsafe,
            functions_total,
            expressions_unsafe,
            expressions_total,
            impls_unsafe,
            impls_total,
            traits_unsafe,
            traits_total,
            methods_unsafe,
            methods_total,
            ptr_derefs_unsafe,
            ptr_derefs_total,
            unsafe_calls_unsafe,
            unsafe_calls_total,
            forbids_unsafe,
        })
    }

    fn add(&mut self, other: &Self) {
        self.functions_unsafe += other.functions_unsafe;
        self.functions_total += other.functions_total;
        self.expressions_unsafe += other.expressions_unsafe;
        self.expressions_total += other.expressions_total;
        self.impls_unsafe += other.impls_unsafe;
        self.impls_total += other.impls_total;
        self.traits_unsafe += other.traits_unsafe;
        self.traits_total += other.traits_total;
        self.methods_unsafe += other.methods_unsafe;
        self.methods_total += other.methods_total;
        self.ptr_derefs_unsafe += other.ptr_derefs_unsafe;
        self.ptr_derefs_total += other.ptr_derefs_total;
        self.unsafe_calls_unsafe += other.unsafe_calls_unsafe;
        self.unsafe_calls_total += other.unsafe_calls_total;
    }
}

fn find_cargo_projects(root: &Path) -> Vec<PathBuf> {
    let mut projects = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "Cargo.toml" {
            if let Some(parent) = entry.path().parent() {
                projects.push(parent.to_path_buf());
            }
        }
    }

    projects.sort();
    projects
}

fn analyze_project(project_path: &Path, cargo_geiger: &Path) -> Option<(String, UnsafeStats)> {
    let project_name = project_path.file_name()?.to_str()?;

    println!("Analyzing: {}", project_name);

    // Run cargo-geiger
    let output = Command::new(cargo_geiger)
        .arg("geiger")
        .arg("--quiet")
        .current_dir(project_path)
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("  ✗ Failed to run cargo-geiger: {}", e);
            return None;
        }
    };

    if !output.status.success() {
        eprintln!("  ✗ cargo-geiger failed");
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse the output to extract stats
    // Look for the line with the root package stats
    for line in stdout.lines() {
        // Skip header lines
        if line.contains("Functions") || line.contains("Metric output") || line.contains("Symbols") {
            continue;
        }

        // Look for lines with stats (contain "/" characters and numbers)
        if line.contains('/') && !line.contains("target/") {
            if let Some(stats) = UnsafeStats::parse_from_line(line) {
                println!("  ✓ Analysis complete");
                return Some((project_name.to_string(), stats));
            }
        }
    }

    eprintln!("  ✗ Could not parse output");
    None
}

fn main() {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Unsafe Usage Analysis - All Cargo Projects");
    println!("═══════════════════════════════════════════════════════════\n");

    let source_dir = Path::new("/home/crates/source");
    let cargo_geiger = Path::new("/home/crates/source/cargo-fingrain-geiger/target/debug/cargo-geiger");

    // Find all projects
    println!("Discovering cargo projects...");
    let projects = find_cargo_projects(source_dir);
    println!("Found {} cargo projects\n", projects.len());

    println!("Starting analysis...\n");

    let mut results: HashMap<String, UnsafeStats> = HashMap::new();
    let mut total_stats = UnsafeStats::default();
    let mut successful = 0;
    let mut failed = 0;

    for project in &projects {
        if let Some((name, stats)) = analyze_project(project, cargo_geiger) {
            total_stats.add(&stats);
            results.insert(name, stats);
            successful += 1;
        } else {
            failed += 1;
        }
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Analysis Complete");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("Successfully analyzed: {}", successful);
    println!("Failed to analyze: {}\n", failed);

    println!("═══════════════════════════════════════════════════════════");
    println!("  AGGREGATE STATISTICS");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("┌─────────────────────┬──────────┬──────────┬────────────┐");
    println!("│ Metric              │ Unsafe   │ Total    │ Percentage │");
    println!("├─────────────────────┼──────────┼──────────┼────────────┤");

    let print_metric = |name: &str, unsafe_count: u64, total: u64| {
        let percentage = if total > 0 {
            (unsafe_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        println!("│ {:<19} │ {:>8} │ {:>8} │ {:>9.2}% │",
            name, unsafe_count, total, percentage);
    };

    print_metric("Functions", total_stats.functions_unsafe, total_stats.functions_total);
    print_metric("Expressions", total_stats.expressions_unsafe, total_stats.expressions_total);
    print_metric("Impls", total_stats.impls_unsafe, total_stats.impls_total);
    print_metric("Traits", total_stats.traits_unsafe, total_stats.traits_total);
    print_metric("Methods", total_stats.methods_unsafe, total_stats.methods_total);
    print_metric("Ptr Derefs", total_stats.ptr_derefs_unsafe, total_stats.ptr_derefs_total);
    print_metric("Unsafe Calls", total_stats.unsafe_calls_unsafe, total_stats.unsafe_calls_total);

    println!("└─────────────────────┴──────────┴──────────┴────────────┘\n");

    // Total unsafe usage
    let total_unsafe = total_stats.functions_unsafe
        + total_stats.expressions_unsafe
        + total_stats.impls_unsafe
        + total_stats.traits_unsafe
        + total_stats.methods_unsafe
        + total_stats.ptr_derefs_unsafe
        + total_stats.unsafe_calls_unsafe;

    let total_items = total_stats.functions_total
        + total_stats.expressions_total
        + total_stats.impls_total
        + total_stats.traits_total
        + total_stats.methods_total
        + total_stats.ptr_derefs_total
        + total_stats.unsafe_calls_total;

    println!("TOTAL UNSAFE USAGE: {} out of {} items ({:.2}%)\n",
        total_unsafe, total_items,
        if total_items > 0 { (total_unsafe as f64 / total_items as f64) * 100.0 } else { 0.0 });

    // Top 10 projects with most unsafe usage
    println!("═══════════════════════════════════════════════════════════");
    println!("  TOP 10 PROJECTS BY UNSAFE USAGE");
    println!("═══════════════════════════════════════════════════════════\n");

    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| {
        let a_unsafe = a.1.functions_unsafe + a.1.expressions_unsafe + a.1.ptr_derefs_unsafe;
        let b_unsafe = b.1.functions_unsafe + b.1.expressions_unsafe + b.1.ptr_derefs_unsafe;
        b_unsafe.cmp(&a_unsafe)
    });

    for (i, (name, stats)) in sorted_results.iter().take(10).enumerate() {
        let unsafe_count = stats.functions_unsafe + stats.expressions_unsafe + stats.ptr_derefs_unsafe;
        println!("{}. {} - {} unsafe items", i + 1, name, unsafe_count);
        println!("   Functions: {}/{}, Expressions: {}/{}, Ptr Derefs: {}/{}",
            stats.functions_unsafe, stats.functions_total,
            stats.expressions_unsafe, stats.expressions_total,
            stats.ptr_derefs_unsafe, stats.ptr_derefs_total);
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Analysis saved to: unsafe_analysis_results.txt");
    println!("═══════════════════════════════════════════════════════════");

    // Save detailed results
    let mut output = String::new();
    output.push_str("DETAILED UNSAFE USAGE ANALYSIS\n");
    output.push_str("═══════════════════════════════════════════════════════════\n\n");

    for (name, stats) in sorted_results {
        output.push_str(&format!("Project: {}\n", name));
        output.push_str(&format!("  Functions:    {}/{}\n", stats.functions_unsafe, stats.functions_total));
        output.push_str(&format!("  Expressions:  {}/{}\n", stats.expressions_unsafe, stats.expressions_total));
        output.push_str(&format!("  Impls:        {}/{}\n", stats.impls_unsafe, stats.impls_total));
        output.push_str(&format!("  Traits:       {}/{}\n", stats.traits_unsafe, stats.traits_total));
        output.push_str(&format!("  Methods:      {}/{}\n", stats.methods_unsafe, stats.methods_total));
        output.push_str(&format!("  Ptr Derefs:   {}/{}\n", stats.ptr_derefs_unsafe, stats.ptr_derefs_total));
        output.push_str(&format!("  Unsafe Calls: {}/{}\n", stats.unsafe_calls_unsafe, stats.unsafe_calls_total));
        output.push_str(&format!("  Forbids Unsafe: {}\n", if stats.forbids_unsafe { "Yes" } else { "No" }));
        output.push_str("\n");
    }

    fs::write("unsafe_analysis_results.txt", output).ok();
}
