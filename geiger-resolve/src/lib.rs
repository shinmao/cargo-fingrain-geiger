//! geiger-resolve ☢
//! ================
//!
//! This crate provides rustc-based semantic analysis for cargo-geiger.
//! It uses rustc internal APIs to properly resolve unsafe function calls
//! and classify them by their origin (core, alloc, std, or other crates).
//!
//! **Note:** This crate requires nightly Rust with rustc_private feature.

#![forbid(unsafe_code)]
#![deny(warnings)]
#![cfg_attr(feature = "rustc_private", feature(rustc_private))]

#[cfg(feature = "rustc_private")]
mod callbacks;
#[cfg(feature = "rustc_private")]
mod visitor;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Record of a single unsafe function call with its origin classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsafeCallRecord {
    /// Name of the crate containing this call
    pub crate_name: String,
    /// Source file path
    pub file: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Full qualified path of the callee (e.g., "core::ptr::read")
    pub callee_full_path: String,
    /// Crate name where the callee is defined
    pub callee_crate: String,
    /// Origin classification: "Core", "Alloc", "Std", or "Other"
    pub origin_kind: OriginKind,
}

/// Classification of unsafe function origin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OriginKind {
    /// From the `core` crate
    Core,
    /// From the `alloc` crate
    Alloc,
    /// From the `std` crate
    Std,
    /// From any other crate (user code or dependencies)
    Other,
}

impl OriginKind {
    /// Classify based on crate name
    pub fn from_crate_name(crate_name: &str) -> Self {
        match crate_name {
            "core" => OriginKind::Core,
            "alloc" => OriginKind::Alloc,
            "std" => OriginKind::Std,
            _ => OriginKind::Other,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            OriginKind::Core => "Core",
            OriginKind::Alloc => "Alloc",
            OriginKind::Std => "Std",
            OriginKind::Other => "Other",
        }
    }
}

/// Complete report of all unsafe calls in a crate
#[derive(Debug, Serialize, Deserialize)]
pub struct UnsafeCallReport {
    pub records: Vec<UnsafeCallRecord>,
}

/// Summary of unsafe calls by origin
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnsafeCallSummary {
    /// Total number of unsafe calls from `core` crate
    pub core_calls: u64,
    /// Total number of unsafe calls from `alloc` crate
    pub alloc_calls: u64,
    /// Total number of unsafe calls from `std` crate
    pub std_calls: u64,
    /// Total number of unsafe calls from other crates
    pub other_calls: u64,
}

impl UnsafeCallSummary {
    pub fn total(&self) -> u64 {
        self.core_calls + self.alloc_calls + self.std_calls + self.other_calls
    }
}

impl UnsafeCallReport {
    pub fn new() -> Self {
        UnsafeCallReport {
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: UnsafeCallRecord) {
        self.records.push(record);
    }

    /// Generate a summary of unsafe calls by origin
    pub fn summary(&self) -> UnsafeCallSummary {
        let mut summary = UnsafeCallSummary::default();
        for record in &self.records {
            match record.origin_kind {
                OriginKind::Core => summary.core_calls += 1,
                OriginKind::Alloc => summary.alloc_calls += 1,
                OriginKind::Std => summary.std_calls += 1,
                OriginKind::Other => summary.other_calls += 1,
            }
        }
        summary
    }

    /// Write the report to a JSON file
    pub fn write_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    /// Read a report from a JSON file
    pub fn read_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let report = serde_json::from_reader(file)?;
        Ok(report)
    }
}

impl Default for UnsafeCallReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Main entry point for analyzing unsafe calls in a crate using rustc internals.
///
/// This function requires nightly Rust and rustc_private feature to be enabled.
/// It will compile the crate with custom callbacks that track unsafe function calls.
///
/// # Arguments
///
/// * `rustc_args` - Arguments to pass to rustc (same as would be used by cargo)
/// * `output_path` - Path where the JSON report should be written
///
/// # Returns
///
/// Returns Ok(()) if analysis succeeds, or an error if compilation or analysis fails.
#[cfg(feature = "rustc_private")]
pub fn analyze_unsafe_calls_for_current_crate(
    rustc_args: &[String],
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    extern crate rustc_driver;

    use callbacks::GeigerCallbacks;

    let mut callbacks = GeigerCallbacks::new(output_path.clone());

    let exit_code = rustc_driver::catch_with_exit_code(move || {
        rustc_driver::run_compiler(rustc_args, &mut callbacks)
    });

    if exit_code == 0 {
        Ok(())
    } else {
        Err(format!("Rustc compilation failed with exit code: {}", exit_code).into())
    }
}

/// Fallback stub when rustc_private is not available
#[cfg(not(feature = "rustc_private"))]
pub fn analyze_unsafe_calls_for_current_crate(
    _rustc_args: &[String],
    _output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("Rustc-based analysis requires nightly Rust with rustc_private feature. \
         Please compile with nightly and enable the rustc_private feature.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_kind_classification() {
        assert_eq!(OriginKind::from_crate_name("core"), OriginKind::Core);
        assert_eq!(OriginKind::from_crate_name("alloc"), OriginKind::Alloc);
        assert_eq!(OriginKind::from_crate_name("std"), OriginKind::Std);
        assert_eq!(OriginKind::from_crate_name("my_crate"), OriginKind::Other);
    }

    #[test]
    fn test_unsafe_call_report_serialization() {
        let mut report = UnsafeCallReport::new();
        report.add_record(UnsafeCallRecord {
            crate_name: "test_crate".to_string(),
            file: "src/lib.rs".to_string(),
            line: 10,
            column: 5,
            callee_full_path: "core::ptr::read".to_string(),
            callee_crate: "core".to_string(),
            origin_kind: OriginKind::Core,
        });

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("core::ptr::read"));
        assert!(json.contains("Core"));
    }
}
