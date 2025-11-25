# Rustc-Based Unsafe Function Call Analysis - Implementation Guide

## Overview

This document describes how to implement the rustc_private-based analysis in `geiger-resolve` to properly track unsafe function calls and classify them by origin (core, alloc, std, other).

## Architecture

The implementation uses `rustc_driver` to run a custom compiler pass that:
1. Traverses the HIR (High-level Intermediate Representation)
2. Tracks unsafe contexts (unsafe blocks, unsafe functions)
3. Identifies function calls in unsafe contexts
4. Resolves the DefId of each callee using type information
5. Extracts the full definition path and crate name
6. Classifies by origin and records location information

## Required Dependencies

Add to `geiger-resolve/Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

[target.'cfg(not(windows))'.dependencies]
# These are rustc internal crates - only available with nightly
rustc_driver = { optional = true }
rustc_interface = { optional = true }
rustc_hir = { optional = true }
rustc_middle = { optional = true }
rustc_span = { optional = true }

[features]
rustc_private = []
```

Note: `rustc_private` crates are not published to crates.io and require nightly Rust with special configuration.

## Implementation Steps

### 1. Create HIR Visitor

In `geiger-resolve/src/visitor.rs`:

```rust
use rustc_hir as hir;
use rustc_hir::intravisit::{self, Visitor};
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

pub struct UnsafeCallCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    records: Vec<UnsafeCallRecord>,
    unsafe_depth: u32,
}

impl<'tcx> UnsafeCallCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        UnsafeCallCollector {
            tcx,
            records: Vec::new(),
            unsafe_depth: 0,
        }
    }

    pub fn into_records(self) -> Vec<UnsafeCallRecord> {
        self.records
    }

    fn in_unsafe_context(&self) -> bool {
        self.unsafe_depth > 0
    }
}

impl<'tcx> Visitor<'tcx> for UnsafeCallCollector<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_fn(&mut self, fk: intravisit::FnKind<'tcx>, fd: &'tcx hir::FnDecl<'tcx>, b: hir::BodyId, s: Span, id: hir::HirId) {
        // Check if function is unsafe
        let is_unsafe = matches!(fk, intravisit::FnKind::ItemFn(_, _, hir::FnHeader { unsafety: hir::Unsafety::Unsafe, .. }) |
                                     intravisit::FnKind::Method(_, hir::FnSig { header: hir::FnHeader { unsafety: hir::Unsafety::Unsafe, .. }, .. }));

        if is_unsafe {
            self.unsafe_depth += 1;
        }

        intravisit::walk_fn(self, fk, fd, b, s, id);

        if is_unsafe {
            self.unsafe_depth -= 1;
        }
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        match &expr.kind {
            hir::ExprKind::Block(block, _) if block.rules == hir::BlockCheckMode::UnsafeBlock(_) => {
                self.unsafe_depth += 1;
                intravisit::walk_expr(self, expr);
                self.unsafe_depth -= 1;
                return;
            }
            hir::ExprKind::Call(func, _args) if self.in_unsafe_context() => {
                self.process_call(expr, func);
            }
            hir::ExprKind::MethodCall(_path, _receiver, _args, _span) if self.in_unsafe_context() => {
                self.process_method_call(expr);
            }
            _ => {}
        }

        intravisit::walk_expr(self, expr);
    }

    fn process_call(&mut self, expr: &'tcx hir::Expr<'tcx>, func: &'tcx hir::Expr<'tcx>) {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner);

        if let Some(def_id) = typeck_results.type_dependent_def_id(func.hir_id) {
            let full_path = self.tcx.def_path_str(def_id);
            let crate_name = self.tcx.crate_name(def_id.krate).to_string();
            let origin_kind = OriginKind::from_crate_name(&crate_name);

            let source_map = self.tcx.sess.source_map();
            let pos = source_map.lookup_char_pos(expr.span.lo());

            self.records.push(UnsafeCallRecord {
                crate_name: self.tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string(),
                file: pos.file.name.prefer_local().to_string(),
                line: pos.line as u32,
                column: pos.col.0 as u32,
                callee_full_path: full_path,
                callee_crate: crate_name,
                origin_kind,
            });
        }
    }

    fn process_method_call(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner);

        if let Some(def_id) = typeck_results.type_dependent_def_id(expr.hir_id) {
            // Similar to process_call...
        }
    }
}
```

### 2. Create Compiler Callbacks

In `geiger-resolve/src/callbacks.rs`:

```rust
use rustc_driver::Callbacks;
use rustc_interface::interface::Compiler;
use rustc_interface::Queries;

pub struct GeigerCallbacks {
    output_path: PathBuf,
}

impl GeigerCallbacks {
    pub fn new(output_path: PathBuf) -> Self {
        GeigerCallbacks { output_path }
    }
}

impl Callbacks for GeigerCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let mut visitor = UnsafeCallCollector::new(tcx);

            let hir = tcx.hir();
            hir.visit_all_item_likes_in_crate(&mut visitor);

            let records = visitor.into_records();
            let report = UnsafeCallReport { records };

            report.write_to_file(&self.output_path).unwrap();
        });

        rustc_driver::Compilation::Continue
    }
}
```

### 3. Update Main Entry Point

In `geiger-resolve/src/lib.rs`, implement the analyze function:

```rust
#[cfg(feature = "rustc_private")]
pub fn analyze_unsafe_calls_for_current_crate(
    rustc_args: &[String],
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    use rustc_driver::RunCompiler;

    let callbacks = GeigerCallbacks::new(output_path.clone());

    let result = RunCompiler::new(rustc_args, Box::new(callbacks)).run();

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Rustc compilation failed".into()),
    }
}
```

## Building and Testing

### Building with Nightly

```bash
# Install nightly Rust
rustup toolchain install nightly

# Build with rustc_private feature
cargo +nightly build --features rustc_private -p geiger-resolve
```

### Testing

Create test fixtures in `geiger-resolve/tests/fixtures/`:

```rust
// tests/fixtures/test_unsafe_calls.rs
pub unsafe fn f1(p: *const u8) {
    core::ptr::read(p);  // Should be detected as Core
}

pub unsafe fn f2<T>(p: *mut T, len: usize) -> Vec<T> {
    Vec::from_raw_parts(p, len, len)  // Should be detected as Alloc
}

pub unsafe fn f3(p: *const u8) {
    std::ptr::read(p);  // Should be detected as Std
}
```

## Integration with cargo-geiger

1. Add `--with-origin-analysis` CLI flag
2. When enabled, invoke rustc via `geiger-resolve::analyze_unsafe_calls_for_current_crate`
3. Parse the JSON output
4. Augment the existing report with origin information

## Limitations

- Requires nightly Rust
- Only works when rustc_private feature is enabled
- Cannot analyze pre-compiled dependencies
- Type resolution is required, so code must compile

## References

- [rustc-dev-guide: HIR](https://rustc-dev-guide.rust-lang.org/hir.html)
- [rustc-dev-guide: ty module](https://rustc-dev-guide.rust-lang.org/ty.html)
- [clippy source](https://github.com/rust-lang/rust-clippy/) - Good example of using rustc internals
