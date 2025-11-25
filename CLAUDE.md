# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

cargo-geiger is a cargo plugin that detects and reports unsafe Rust code usage in a Rust crate and its dependencies. It scans Rust source files using AST analysis (via syn) to count unsafe usage patterns and produces reports in various formats.

## Workspace Structure

This is a Cargo workspace with four crates:

- `cargo-geiger/`: Main binary crate implementing the CLI tool
- `geiger/`: Library crate providing core scanning functionality, decoupled from cargo
- `cargo-geiger-serde/`: Library crate containing serializable report types
- `geiger-resolve/`: Library crate for rustc-based semantic analysis of unsafe function calls (requires nightly)

Test fixtures are in `test_crates/` and excluded from the workspace build.

## Build and Test Commands

```bash
# Build all workspace members
cargo build

# Build with vendored OpenSSL (statically linked)
cargo build --features vendored-openssl

# Run all tests (with specific test exclusions used by CI)
cargo test -- --skip args::args_tests::update_config_test_color_choice::case_4 --skip test_package

# Run tests for a specific crate
cargo test -p cargo-geiger
cargo test -p geiger
cargo test -p cargo-geiger-serde

# Run a single test by name
cargo test <test_name>

# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run the tool locally
cargo run -- geiger [OPTIONS]
```

## Code Standards

- **Unsafe code is forbidden**: All source files use `#![forbid(unsafe_code)]`
- **Warnings are denied**: `#![deny(warnings)]` and `RUSTFLAGS='-D warnings'` in CI
- **Formatting**: Use `cargo fmt` before committing
- **Clippy**: Recommendations should be applied (though currently not enforced in CI)
- **MSRV**: Rust 1.85 (specified in cargo-geiger/Cargo.toml)

## Architecture

### Scanning Flow

1. **CLI Entry** (cargo-geiger/src/main.rs): Parse arguments and initialize cargo GlobalContext
2. **Metadata Loading** (cargo-geiger/src/cli.rs): Load cargo metadata and build krates dependency graph
3. **Graph Building** (cargo-geiger/src/graph.rs): Construct petgraph representation of package dependencies, respecting target platform and dependency types (dev/build/normal)
4. **Scanning** (cargo-geiger/src/scan.rs):
   - Two modes: full scan (default) or forbid-only (faster, entry points only)
   - Traverses dependency graph and scans each .rs file
   - Uses geiger crate for actual file analysis
5. **Output** (cargo-geiger/src/format.rs, cargo-geiger/src/tree.rs): Format results as tree, JSON, or table; optionally write to README.md

### Core Scanning Logic (geiger crate)

- `geiger/src/find.rs`: Entry point for scanning files
- `geiger/src/geiger_syn_visitor.rs`: syn::visit::Visit implementation that walks the AST counting:
  - Unsafe functions/methods
  - Unsafe blocks (expressions)
  - Unsafe trait implementations
  - Unsafe trait definitions
  - Functions with unsafe attributes (`#[no_mangle]`, `#[export_name]`)
  - Detects `#![forbid(unsafe_code)]` attribute
  - Optionally includes/excludes test code

### Report Types (cargo-geiger-serde)

Serializable structures for report output:
- `CounterBlock`: Counts of safe vs unsafe for functions, expressions, impls, traits, methods, ptr_derefs, unsafe_fn_calls
- `SafetyReport` / `QuickSafetyReport`: Full dependency tree reports
- `PackageInfo` / `UnsafeInfo`: Per-package unsafe usage information

### Enhanced Tracking

The geiger visitor now tracks two additional unsafe patterns:

1. **Raw Pointer Dereferences** (`ptr_derefs`): Detects `*ptr` dereferences inside unsafe blocks or unsafe functions
2. **Unsafe Function Calls** (`unsafe_fn_calls`): Placeholder for future rustc-based analysis that will classify unsafe function calls by origin

### Rustc-Based Analysis (geiger-resolve)

The `geiger-resolve` crate provides infrastructure for compiler-based semantic analysis to properly track and classify unsafe function calls:

- Uses `rustc_private` APIs to access type information
- Classifies unsafe calls by origin: Core, Alloc, Std, or Other crates
- Requires nightly Rust with `rustc_private` feature
- See `geiger-resolve/IMPLEMENTATION.md` for implementation guide

## Key Design Decisions

- Uses `krates` crate for dependency graph management instead of direct cargo APIs where possible
- Supports filtering by target platform using cargo's platform resolution
- Can scan with different dependency scopes (dev, build, normal, or all)
- Forbid-only mode skips building dependencies and only checks entry points for `#![forbid(unsafe_code)]`
- Test code can be included or excluded from counts via `--include-tests` flag

## Common Development Patterns

When working with dependency resolution, use `CargoMetadataParameters` which bundles `cargo_metadata::Metadata` and `Krates` together. Most functions take this as a parameter to access both cargo metadata and the resolved dependency graph.

The `Graph` struct (cargo-geiger/src/graph.rs) wraps a `petgraph::Graph<PackageId, DependencyKind>` and maintains a `HashMap<PackageId, NodeIndex>` for efficient lookups.

Metrics are accumulated in `GeigerContext` which maps `PackageId` to `PackageMetrics`, and each `PackageMetrics` maps canonical paths to `RsFileMetricsWrapper`.
