Below is a polished **English engineering specification** you can paste directly into **Claude Code** as your agent instruction.

It is written to be **actionable**, **complete**, and **fully detailed**, so Claude can implement the feature directly inside **cargo-geiger**.

---

# **Claude Code – Development Instruction for Extending `cargo-geiger`**

## **Goal**

Extend `cargo-geiger` so that unsafe function calls can be classified based on their *origin*:

* Calls originating from **`core`**
* Calls originating from **`alloc`**
* Calls originating from **`std`**
* Calls originating from **other crates**

To accomplish this, implement a **rustc-based semantic analysis backend** capable of producing the **full definition path** of each unsafe function call, such as:

* `core::ptr::read`
* `std::ptr::NonNull::<T>::new_unchecked`
* `alloc::vec::from_raw_parts`
* `my_crate::module::foo`

This backend will complement (but not replace) the existing syn-based analysis.

---

# **High-Level Architecture**

1. Add a new internal crate or module, tentatively called `geiger_resolve`, which uses:

   * `rustc_driver`
   * `rustc_interface`
   * `rustc_hir`
   * `rustc_middle`
   * `rustc_span`

2. This module runs a customized rustc frontend using `RunCompiler` and captures:

   * Every **unsafe block**
   * Every **unsafe function call** inside that context
   * The **callee's DefId**
   * The **full definition path** via `tcx.def_path_str(def_id)`
   * The **origin crate name** via `tcx.crate_name(def_id.krate)`
   * The **source location** (file, line, column)

3. The backend outputs a JSON report listing all unsafe calls with their origin classification:

   * `"Core"`
   * `"Alloc"`
   * `"Std"`
   * `"Other"`

4. `cargo-geiger` will add a new CLI flag to enable the enhanced analysis:

   ```
   --with-origin-analysis
   ```

5. When this flag is enabled, the geiger report is augmented with origin information for each unsafe call.

---

# **Development Tasks (for Claude)**

### **1. Add new crate `geiger-resolve`**

Add to workspace:

```toml
[workspace]
members = [
    "cargo-geiger",
    "geiger-resolve"
]
```

Create `geiger-resolve/Cargo.toml`:

```toml
[package]
name = "geiger-resolve"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

rustc_driver = { package = "rustc_driver", version = "0.0.0", features = ["rustc_private"] }
rustc_interface = { package = "rustc_interface", version = "0.0.0" }
rustc_hir = { package = "rustc_hir", version = "0.0.0" }
rustc_middle = { package = "rustc_middle", version = "0.0.0" }
rustc_span = { package = "rustc_span", version = "0.0.0" }
```

The crate must compile using nightly and support `rustc_private`.

---

### **2. Implement JSON output data structures**

In `geiger-resolve/src/lib.rs`:

```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct UnsafeCallRecord {
    pub crate_name: String,
    pub file: String,
    pub line: u32,
    pub column: u32,

    pub callee_full_path: String,
    pub callee_crate: String,
    pub origin_kind: String, // "Core" | "Alloc" | "Std" | "Other"
}

#[derive(Serialize)]
pub struct UnsafeCallReport {
    pub records: Vec<UnsafeCallRecord>,
}
```

---

### **3. Implement rustc driver callback using `RunCompiler`**

Add:

```rust
pub fn analyze_unsafe_calls_for_current_crate(
    rustc_args: &[String],
    output_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>>;
```

Implement a `Callbacks` struct that:

* Executes after `analysis`
* Traverses HIR
* Visits each function, block, and expression
* Tracks unsafe context
* Detects call expressions
* Uses `tcx.typeck(expr.hir_id.owner)` to get callee `DefId`

Full path obtained via:

```rust
let full_path = tcx.def_path_str(def_id);
```

Origin crate:

```rust
let crate_name = tcx.crate_name(def_id.krate).to_string();
```

Classify:

```rust
let origin_kind = match crate_name.as_str() {
    "core" => "Core",
    "alloc" => "Alloc",
    "std" => "Std",
    _ => "Other",
};
```

Capture source location:

```rust
let sm = tcx.sess.source_map();
let pos = sm.lookup_char_pos(expr.span.lo());
```

Push record into sink vector.

Finally, serialize:

```rust
serde_json::to_writer_pretty(file, &report)?;
```

---

### **4. Integrate with cargo-geiger CLI**

Modify the CLI (likely in `cargo-geiger/src/cmd/...`) to add:

```rust
#[clap(long)]
pub with_origin_analysis: bool,
```

When enabled:

1. Construct the exact rustc arguments that Cargo would have used

   * You may reuse existing `RUSTC_WRAPPER` mechanics or cargo-geiger’s compilation hooks
2. Execute:

```rust
geiger_resolve::analyze_unsafe_calls_for_current_crate(&rustc_args, &output_json_path)
```

3. Read JSON back into a structure in the main `cargo-geiger` crate
4. Augment the existing unsafe report with:

   * Full function path
   * Origin classification (`Core`, `Alloc`, `Std`, `Other`)

---

### **5. Testing and validation**

Add test crates (fixtures) such as:

```rust
pub unsafe fn f1(p: *const u8) {
    core::ptr::read(p);
}

pub unsafe fn f2<T>(p: *mut T, len: usize) -> Vec<T> {
    Vec::from_raw_parts(p, len, len)
}

pub unsafe fn f3(p: *const u8) {
    std::ptr::read(p);
}
```

Expected classifications:

| Function                     | Origin |
| ---------------------------- | ------ |
| `core::ptr::read`            | Core   |
| `alloc::vec::from_raw_parts` | Alloc  |
| `std::ptr::read`             | Std    |

---

# **Key Requirements for Claude**

1. Use **rustc_private API** to retrieve **DefId** and **full function path**
2. Never attempt to resolve paths using syn alone
   (syn cannot resolve module paths or trait methods)
3. Preserve existing cargo-geiger behavior
4. Add origin classification cleanly into the report
5. Ensure the analysis runs only when the flag is provided
6. Code must build and run with nightly toolchain

---

# Output Expectation

After implementing, running:

```
cargo geiger --with-origin-analysis
```

Should produce a report including fields such as:

```json
{
  "file": "src/lib.rs",
  "line": 12,
  "column": 8,
  "callee_full_path": "core::ptr::read",
  "callee_crate": "core",
  "origin_kind": "Core"
}
```

And similar entries for `alloc` and `std`.

---


