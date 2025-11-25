//! Integration with geiger-resolve for rustc-based unsafe call analysis

#[cfg(feature = "unsafe-call-analysis")]
use crate::mapping::CargoMetadataParameters;
#[cfg(feature = "unsafe-call-analysis")]
use crate::scan::GeigerContext;
#[cfg(feature = "unsafe-call-analysis")]
use cargo::GlobalContext;
#[cfg(feature = "unsafe-call-analysis")]
use cargo_geiger_serde::Count;
#[cfg(feature = "unsafe-call-analysis")]
use std::collections::HashMap;
#[cfg(feature = "unsafe-call-analysis")]
use std::path::PathBuf;
#[cfg(feature = "unsafe-call-analysis")]
use std::process::Command;

/// Runs rustc-based analysis on all packages and merges results into GeigerContext
#[cfg(feature = "unsafe-call-analysis")]
pub fn run_rustc_analysis(
    cargo_metadata_parameters: &CargoMetadataParameters,
    geiger_context: &mut GeigerContext,
    gctx: &GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    use geiger_resolve::{UnsafeCallReport, UnsafeCallSummary};

    // Get the workspace root (for potential future use)
    let _workspace_root = cargo_metadata_parameters
        .metadata
        .workspace_root
        .as_std_path();

    // Try to find geiger-rustc binary
    let geiger_rustc_path = find_geiger_rustc()?;

    // Build a map from package name to source paths
    let mut package_summaries: HashMap<String, UnsafeCallSummary> = HashMap::new();

    // For each package in the metadata, try to run analysis
    for package in cargo_metadata_parameters.metadata.packages.iter() {
        // Skip packages not in the workspace (external dependencies)
        // We can only analyze source code we have access to
        let package_path = package.manifest_path.parent();
        if package_path.is_none() {
            continue;
        }

        let package_path = package_path.unwrap();

        // Find the main source file
        let src_path = if package_path.join("src/lib.rs").exists() {
            package_path.join("src/lib.rs")
        } else if package_path.join("src/main.rs").exists() {
            package_path.join("src/main.rs")
        } else {
            continue;
        };

        // Create a temp file for the analysis output
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join(format!(
            "geiger_rustc_{}_{}.json",
            package.name,
            std::process::id()
        ));

        // Run geiger-rustc on this package
        let result = run_geiger_rustc_on_package(
            &geiger_rustc_path,
            &src_path.as_std_path().to_path_buf(),
            &package.name,
            &output_path,
            gctx,
        );

        if let Ok(()) = result {
            // Try to read the report
            if let Ok(report) = UnsafeCallReport::read_from_file(&output_path) {
                let summary = report.summary();
                package_summaries.insert(package.name.clone(), summary);
            }
        }

        // Clean up temp file
        std::fs::remove_file(&output_path).ok();
    }

    // Now merge the summaries into the geiger context
    for (package_id, package_metrics) in
        geiger_context.package_id_to_metrics.iter_mut()
    {
        // Extract package name from the package_id
        let package_name = extract_package_name(&package_id.repr);

        if let Some(summary) = package_summaries.get(&package_name) {
            // Update the package metrics with unsafe call counts by origin
            for (_path, metrics_wrapper) in
                package_metrics.rs_path_to_metrics.iter_mut()
            {
                // Update the total unsafe_fn_calls
                metrics_wrapper.metrics.counters.unsafe_fn_calls = Count {
                    safe: 0,
                    unsafe_: summary.total(),
                };
                // Update the categorized unsafe calls
                metrics_wrapper.metrics.counters.unsafe_fn_calls_core = Count {
                    safe: 0,
                    unsafe_: summary.core_calls,
                };
                metrics_wrapper.metrics.counters.unsafe_fn_calls_alloc = Count {
                    safe: 0,
                    unsafe_: summary.alloc_calls,
                };
                metrics_wrapper.metrics.counters.unsafe_fn_calls_std = Count {
                    safe: 0,
                    unsafe_: summary.std_calls,
                };
                metrics_wrapper.metrics.counters.unsafe_fn_calls_other = Count {
                    safe: 0,
                    unsafe_: summary.other_calls,
                };
                break; // Only update once per package for now
            }
        }
    }

    Ok(())
}

/// Find the geiger-rustc binary
#[cfg(feature = "unsafe-call-analysis")]
fn find_geiger_rustc() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // First, try to find it in the same directory as the current executable
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            let geiger_rustc = exe_dir.join("geiger-rustc");
            if geiger_rustc.exists() {
                return Ok(geiger_rustc);
            }
        }
    }

    // Try to find it in PATH
    if let Ok(output) = Command::new("which").arg("geiger-rustc").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    // Try cargo target directory
    let cargo_target = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));

    let debug_path = cargo_target.join("debug/geiger-rustc");
    if debug_path.exists() {
        return Ok(debug_path);
    }

    let release_path = cargo_target.join("release/geiger-rustc");
    if release_path.exists() {
        return Ok(release_path);
    }

    Err("geiger-rustc binary not found. Build it with: \
         cargo build --bin geiger-rustc --features rustc_private"
        .into())
}

/// Run geiger-rustc on a single package
#[cfg(feature = "unsafe-call-analysis")]
fn run_geiger_rustc_on_package(
    geiger_rustc_path: &PathBuf,
    src_path: &PathBuf,
    crate_name: &str,
    output_path: &PathBuf,
    _gctx: &GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(geiger_rustc_path)
        .arg("--crate-name")
        .arg(crate_name)
        .arg(src_path)
        .arg("--geiger-output")
        .arg(output_path)
        .arg("--edition")
        .arg("2021")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("geiger-rustc failed: {}", stderr).into());
    }

    Ok(())
}

/// Extract package name from package_id repr string
#[cfg(feature = "unsafe-call-analysis")]
fn extract_package_name(package_id_repr: &str) -> String {
    // Package ID format is typically "name version (source)"
    // e.g., "my_crate 0.1.0 (path+file:///...)"
    package_id_repr
        .split_whitespace()
        .next()
        .unwrap_or(package_id_repr)
        .to_string()
}

/// Stub implementation when feature is not enabled
#[cfg(not(feature = "unsafe-call-analysis"))]
pub fn run_rustc_analysis(
    _cargo_metadata_parameters: &crate::mapping::CargoMetadataParameters,
    _geiger_context: &mut crate::scan::GeigerContext,
    _gctx: &cargo::GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!(
        "Warning: --unsafe-call-analysis flag was used but cargo-geiger was not built with the \
        'unsafe-call-analysis' feature. Please rebuild with: cargo build --features unsafe-call-analysis"
    );
    Ok(())
}
