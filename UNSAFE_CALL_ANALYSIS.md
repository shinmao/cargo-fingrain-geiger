# Unsafe Call Analysis Integration Guide

## Overview

cargo-geiger now supports rustc-based unsafe function call analysis that categorizes unsafe calls by their origin: `core`, `alloc`, `std`, or other crates. This feature provides deeper insight into where unsafe code is being used in your dependencies.

## Current Status

✅ **Phase 1 Complete**: Raw pointer dereference tracking (syn-based, works out of the box)
✅ **Phase 2 Complete**: Rustc-based infrastructure (geiger-resolve crate)
✅ **Phase 3 Complete**: CLI integration (--unsafe-call-analysis flag)
🔧 **Phase 4 In Progress**: Full rustc analysis tooling

## Output Columns

cargo-geiger now displays two additional columns:

- **Ptr Derefs**: Raw pointer dereferences (`*ptr`) detected in unsafe blocks
- **Unsafe Calls**: Calls to unsafe functions categorized by origin

```
Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Dependency
1/1        2/2          0/0    0/0     0/0      0/0         0/0           ! my_crate 0.1.0
```

## Using Pointer Dereference Tracking (Available Now)

This feature works out of the box with no additional setup:

```bash
cargo build
cargo install --path cargo-geiger
cd your-project
cargo geiger
```

The "Ptr Derefs" column will show counts of `*ptr` operations in unsafe blocks.

## Using Unsafe Call Analysis

### Architecture

Due to rustc_private limitations (stdlib conflicts), the unsafe call analysis consists of two components:

1. **cargo-geiger**: Main CLI tool with `--unsafe-call-analysis` flag
2. **geiger-rustc**: Separate binary tool that performs rustc-based analysis

### Current Limitations

The `--unsafe-call-analysis` flag is currently a placeholder for future integration. The infrastructure is in place, but full automation is still in development.

### Manual Workflow (Development/Testing)

If you want to experiment with rustc-based analysis:

#### Step 1: Build the geiger-rustc tool

```bash
cd geiger-resolve
cargo build --bin geiger-rustc --features rustc_private --release
```

#### Step 2: Run analysis on your crate

```bash
# For a single file
./target/release/geiger-rustc \
    --crate-name my_crate \
    src/lib.rs \
    --geiger-output report.json

# The report will be written to report.json
```

#### Step 3: Inspect the report

```bash
cat report.json
```

The report will contain entries like:

```json
{
  "records": [
    {
      "crate_name": "my_crate",
      "file": "src/lib.rs",
      "line": 10,
      "column": 5,
      "callee_full_path": "core::ptr::read",
      "callee_crate": "core",
      "origin_kind": "Core"
    }
  ]
}
```

### Integration with cargo-geiger (Future)

The goal is for `cargo geiger --unsafe-call-analysis` to automatically:

1. Invoke `geiger-rustc` on each package during the build
2. Aggregate the results into the CounterBlock
3. Display categorized counts in the "Unsafe Calls" column

This requires:
- Proper rustc flag construction for each package
- Handling workspace members
- Graceful error handling
- Performance optimization

## Data Structures

### UnsafeCallRecord

Represents a single unsafe function call:

```rust
pub struct UnsafeCallRecord {
    pub crate_name: String,        // Crate containing the call
    pub file: String,               // Source file
    pub line: u32,                  // Line number
    pub column: u32,                // Column number
    pub callee_full_path: String,   // e.g., "core::ptr::read"
    pub callee_crate: String,       // e.g., "core"
    pub origin_kind: OriginKind,    // Classification
}
```

### OriginKind

Classifies the origin of unsafe calls:

```rust
pub enum OriginKind {
    Core,   // From the `core` crate
    Alloc,  // From the `alloc` crate
    Std,    // From the `std` crate
    Other,  // From any other crate (user code or dependencies)
}
```

### UnsafeCallReport

Container for analysis results:

```rust
pub struct UnsafeCallReport {
    pub records: Vec<UnsafeCallRecord>,
}
```

## Command Reference

### cargo-geiger Commands

```bash
# Standard usage (includes pointer dereference tracking)
cargo geiger

# With unsafe call analysis placeholder (future)
cargo geiger --unsafe-call-analysis

# Show help including new flag
cargo geiger --help
```

### geiger-rustc Commands

```bash
# Analyze a single crate
geiger-rustc \
    --crate-name <name> \
    <source_files> \
    --geiger-output <report.json>

# Example with dependencies
geiger-rustc \
    --crate-name my_crate \
    --extern dep_crate=path/to/dep.rlib \
    src/lib.rs \
    --geiger-output report.json
```

## Development Guide

### Building

```bash
# Build everything (without rustc_private)
cargo build

# Build geiger-rustc with rustc_private
cd geiger-resolve
cargo build --bin geiger-rustc --features rustc_private

# Build cargo-geiger with unsafe-call-analysis feature
cd ..
cargo build -p cargo-geiger --features unsafe-call-analysis
```

### Testing

```bash
# Run all tests
cargo test -- --skip args::args_tests::update_config_test_color_choice::case_4 --skip test_package

# Test geiger-resolve
cd geiger-resolve
cargo test --features rustc_private
```

### Adding rustc Analysis

To complete the integration, implement in `cargo-geiger/src/scan/rustc_resolve.rs`:

1. **Construct rustc arguments** for each package
2. **Invoke geiger-rustc** as a subprocess
3. **Parse the JSON reports**
4. **Aggregate counts** into PackageMetrics
5. **Handle errors** gracefully

Example pseudocode:

```rust
fn run_rustc_analysis(
    cargo_metadata_parameters: &CargoMetadataParameters,
    geiger_context: &mut GeigerContext,
    gctx: &GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    for (package_id, package_metrics) in geiger_context.package_id_to_metrics.iter_mut() {
        // Get package info
        let package = get_package(package_id, cargo_metadata_parameters)?;

        // Build rustc args
        let rustc_args = build_rustc_args(package, gctx)?;

        // Invoke geiger-rustc
        let output_path = invoke_geiger_rustc(&rustc_args)?;

        // Read report
        let report = UnsafeCallReport::read_from_file(&output_path)?;

        // Aggregate into package metrics
        aggregate_unsafe_calls(package_metrics, &report);
    }

    Ok(())
}
```

## Troubleshooting

### "rustc_private feature required" error

Make sure you're using nightly Rust and building with the feature:

```bash
rustc --version  # Should show "nightly"
cargo build --bin geiger-rustc --features rustc_private
```

### "Cannot find geiger-rustc binary"

Build it first:

```bash
cd geiger-resolve
cargo build --bin geiger-rustc --features rustc_private --release
```

The binary will be at `target/release/geiger-rustc`.

### "Analysis failed" error

Check that:
- Your crate compiles successfully
- You're providing correct rustc arguments
- All dependencies are available

## Future Work

1. **Automated invocation**: cargo-geiger automatically runs geiger-rustc
2. **Workspace support**: Handle workspace members correctly
3. **Caching**: Avoid re-analyzing unchanged packages
4. **Parallel analysis**: Analyze multiple packages concurrently
5. **Origin breakdown**: Show separate counts for core/alloc/std/other
6. **CI integration**: Support for continuous monitoring

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                       cargo-geiger                           │
│  ┌────────────────┐         ┌─────────────────────────┐    │
│  │  syn-based     │         │  rustc-based (future)   │    │
│  │  scanning      │         │  analysis integration   │    │
│  │  (geiger crate)│         │  (rustc_resolve.rs)     │    │
│  └────────┬───────┘         └───────────┬─────────────┘    │
│           │                             │                   │
│           │                             │ (invokes)         │
│           │                             ▼                   │
│           │                    ┌──────────────────┐         │
│           │                    │  geiger-rustc    │         │
│           │                    │  (separate bin)  │         │
│           │                    └────────┬─────────┘         │
│           │                             │                   │
│           └─────────────┬───────────────┘                   │
│                         │                                   │
│                         ▼                                   │
│              ┌────────────────────┐                        │
│              │   GeigerContext    │                        │
│              │  (PackageMetrics)  │                        │
│              └──────────┬─────────┘                        │
│                         │                                   │
│                         ▼                                   │
│               ┌──────────────────┐                         │
│               │   Output         │                         │
│               │   (table/json)   │                         │
│               └──────────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

## Contributing

To contribute to unsafe call analysis:

1. **Data structures**: Ready to use (geiger-resolve/src/lib.rs)
2. **HIR visitor**: Implemented (geiger-resolve/src/visitor.rs)
3. **Binary tool**: Basic version complete (geiger-resolve/src/bin/geiger-rustc.rs)
4. **Integration hook**: In place (cargo-geiger/src/scan/rustc_resolve.rs)
5. **Next step**: Implement automated rustc invocation

See the TODO comments in `cargo-geiger/src/scan/rustc_resolve.rs` for specific tasks.

## References

- **RUSTC_IMPLEMENTATION_NOTE.md**: Status and API fixes
- **RUSTC_API_FIXES.md**: Detailed API compatibility changes
- **SESSION_SUMMARY.md**: High-level overview
- **geiger-resolve/README.md**: (if created) Library documentation

## Questions?

- Check existing issues: https://github.com/rust-secure-code/cargo-geiger/issues
- Create a new issue for bugs or feature requests
- Mention that you're using the unsafe call analysis feature
