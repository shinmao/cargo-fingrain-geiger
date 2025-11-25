cargo-fingrain-geiger ☢️ 
========================

A fine-grained unsafe Rust code analyzer that classifies unsafe operations by their origin and type.

This is an enhanced fork of [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) with additional capabilities for analyzing **where** unsafe calls originate from and tracking **all 5 types of unsafe behaviors** defined by Rust.

## Rust's 5 Unsafe Behaviors

According to the Rust Reference, there are exactly 5 operations that require `unsafe`:

1. **Dereference a raw pointer** - `*ptr`
2. **Call an unsafe function or method** - `unsafe_fn()`
3. **Access or modify a mutable static variable** - `static mut`
4. **Implement an unsafe trait** - `unsafe impl Trait`
5. **Access fields of unions** - `union.field`

This tool tracks all of these behaviors (where statically determinable).

## New Features

In addition to the original cargo-geiger functionality, this tool provides:

- **Fine-grained classification of unsafe function/method calls**:
  - `core::` - Calls to Rust core library (e.g., `ptr::read`, `mem::transmute`)
  - `alloc::` - Calls to allocation library (e.g., `Vec::set_len`, `Box::from_raw`)
  - `std::` - Calls to standard library
  - `other` - Calls to crate-defined unsafe functions

- **Detection of unsafe method calls** (not just function calls):
  - Raw pointer methods: `.add()`, `.sub()`, `.offset()`, `.read()`, `.write()`
  - Slice methods: `.get_unchecked()`, `.get_unchecked_mut()`
  - MaybeUninit methods: `.assume_init()`, `.assume_init_ref()`
  - And many more from the standard library

- **SIMD intrinsics detection**:
  - x86/x86_64: `_mm_*`, `_mm256_*`, `_mm512_*`
  - ARM NEON intrinsics

- **Mutable static access tracking** (NEW)

- **Union field access tracking** (NEW, heuristic-based)

## Empirical Study Results

We analyzed **82 popular Rust crates** to understand the composition of unsafe code usage.

### Total Unsafe Operations: 3,029

| Unsafe Behavior | Count | Percentage |
|-----------------|-------|------------|
| **1. Raw Pointer Dereference** | 353 | 11.65% |
| **2. Unsafe Fn/Method Calls** | 2,529 | 83.49% |
| &nbsp;&nbsp;&nbsp;&nbsp;↳ Core calls | 1,387 | 45.79% |
| &nbsp;&nbsp;&nbsp;&nbsp;↳ Alloc calls | 19 | 0.63% |
| &nbsp;&nbsp;&nbsp;&nbsp;↳ Std calls | 16 | 0.53% |
| &nbsp;&nbsp;&nbsp;&nbsp;↳ Other calls | 1,107 | 36.55% |
| **3. Mutable Static Access** | * | * |
| **4. Unsafe Trait Impl** | 144 | 4.75% |
| **4b. Unsafe Trait Declaration** | 3 | 0.10% |
| **5. Union Field Access** | * | * |

\* Tracked but not included in table output format yet

### Key Findings

| Category | Count | Percentage |
|----------|-------|------------|
| Ptr Dereferences | 353 | **11.65%** |
| Stdlib calls (core+alloc+std) | 1,422 | **46.95%** |
| Other unsafe calls | 1,107 | **36.55%** |
| Unsafe trait impls | 144 | **4.75%** |

#### Insights

- **58.60%** of all unsafe operations are either raw pointer dereferences OR calls to standard library functions
  - These are well-documented, auditable unsafe operations
  
- **46.95%** of unsafe calls are to `core::`/`alloc::`/`std::` functions
  - The majority are `core::` calls (45.79%), primarily pointer operations and SIMD intrinsics

- **36.55%** of unsafe calls are to crate-defined (`other`) functions
  - These require more careful manual review

- **~5%** are unsafe trait implementations
  - Often implementing `Send`, `Sync`, or other marker traits

### Raw Data

```
Total unsafe operations analyzed: 3,029
  - Pointer dereferences:        353 (11.65%)
  - Core calls:                1,387 (45.79%)
  - Alloc calls:                  19 (0.63%)
  - Std calls:                    16 (0.53%)
  - Other fn/method calls:     1,107 (36.55%)
  - Unsafe trait impls:          144 (4.75%)
  - Unsafe traits:                 3 (0.10%)
```

## Installation

```bash
git clone https://github.com/shinmao/cargo-fingrain-geiger.git
cd cargo-fingrain-geiger
cargo build --release
```

## Usage

```bash
# Basic usage
cargo geiger

# JSON output with detailed classification
cargo geiger --output-format Json
```

### Output Columns

The tool displays the following metrics:

| Column | Description |
|--------|-------------|
| Functions | Unsafe functions declared |
| Exprs | Expressions in unsafe contexts |
| Impls | Unsafe impl blocks |
| Traits | Unsafe traits |
| Methods | Unsafe methods |
| Ptr Derefs | Raw pointer dereferences |
| Unsafe Calls | Total unsafe fn/method calls |
| Core | Calls to `core::` unsafe APIs |
| Alloc | Calls to `alloc::` unsafe APIs |
| Std | Calls to `std::` unsafe APIs |
| Other | Calls to crate-defined unsafe fns |

## Libraries

This project exposes three libraries:

- `cargo-geiger` - The main binary internals
- `cargo-geiger-serde` - Serializable report types with new classification fields
- `geiger` - Core analysis components with fine-grained unsafe call detection

## Acknowledgments

This project is based on [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) by the Rust Secure Code Working Group.

Original projects that inspired cargo-geiger:
- <https://github.com/icefoxen/cargo-osha>
- <https://github.com/sfackler/cargo-tree>

## Why the name?

<https://en.wikipedia.org/wiki/Geiger_counter>

Unsafe code, like ionizing radiation, is unavoidable in some situations and should be safely contained! This "fine-grained" version helps you understand exactly what kind of radiation you're dealing with. ☢️
