//! Binary tool for rustc-based unsafe call analysis
//!
//! This tool uses rustc_private APIs to analyze unsafe function calls.
//! It must be run with nightly Rust and cannot be linked into regular binaries.
//!
//! Usage:
//!   geiger-rustc <rustc args...> --geiger-output <path>
//!
//! Example:
//!   geiger-rustc --crate-name my_crate src/lib.rs --geiger-output report.json

#![cfg_attr(feature = "rustc_private", feature(rustc_private))]

#[cfg(feature = "rustc_private")]
fn main() {
    use geiger_resolve::analyze_unsafe_calls_for_current_crate;
    use std::env;
    use std::path::PathBuf;

    let args: Vec<String> = env::args().collect();

    // Find --geiger-output flag and separate from rustc args
    let mut output_path = None;
    let mut rustc_args = Vec::new();
    let mut has_crate_type = false;

    // Add program name as first arg (rustc expects this)
    // Use the actual program name from args[0] if available
    let program_name = args.first().cloned().unwrap_or_else(|| "geiger-rustc".to_string());
    rustc_args.push(program_name);

    let mut i = 1; // Skip program name
    while i < args.len() {
        let arg = &args[i];

        if arg == "--geiger-output" {
            if let Some(path) = args.get(i + 1) {
                output_path = Some(PathBuf::from(path));
                i += 2; // Skip both --geiger-output and its value
                continue;
            } else {
                eprintln!("Error: --geiger-output requires a path argument");
                std::process::exit(1);
            }
        }

        // Track if --crate-type is already provided
        if arg == "--crate-type" {
            has_crate_type = true;
        }

        rustc_args.push(arg.clone());
        i += 1;
    }

    let output_path = match output_path {
        Some(p) => p,
        None => {
            eprintln!("Error: --geiger-output <path> is required");
            eprintln!("\nUsage: geiger-rustc <rustc args...> --geiger-output <path>");
            std::process::exit(1);
        }
    };

    // Add required rustc flags if not already present
    if !has_crate_type {
        rustc_args.push("--crate-type".to_string());
        rustc_args.push("lib".to_string());
    }

    match analyze_unsafe_calls_for_current_crate(&rustc_args, &output_path) {
        Ok(()) => {
            println!(
                "Analysis complete. Report written to: {}",
                output_path.display()
            );
        }
        Err(e) => {
            eprintln!("Analysis failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "rustc_private"))]
fn main() {
    eprintln!("Error: geiger-rustc requires the rustc_private feature.");
    eprintln!("Please build with: cargo build --bin geiger-rustc --features rustc_private");
    std::process::exit(1);
}
