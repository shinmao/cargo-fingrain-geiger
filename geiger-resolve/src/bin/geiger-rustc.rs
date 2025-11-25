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

    // Find --geiger-output flag
    let mut output_path = None;
    let mut rustc_args = Vec::new();

    let mut skip_next = false;
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }

        if arg == "--geiger-output" {
            if let Some(path) = args.get(i + 1) {
                output_path = Some(PathBuf::from(path));
                skip_next = true;
            } else {
                eprintln!("Error: --geiger-output requires a path argument");
                std::process::exit(1);
            }
        } else {
            rustc_args.push(arg.clone());
        }
    }

    let output_path = match output_path {
        Some(p) => p,
        None => {
            eprintln!("Error: --geiger-output <path> is required");
            eprintln!("\nUsage: geiger-rustc <rustc args...> --geiger-output <path>");
            std::process::exit(1);
        }
    };

    // Add required rustc flags
    rustc_args.push("--crate-type".to_string());
    rustc_args.push("lib".to_string());

    match analyze_unsafe_calls_for_current_crate(&rustc_args, &output_path) {
        Ok(()) => {
            println!("Analysis complete. Report written to: {}", output_path.display());
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
