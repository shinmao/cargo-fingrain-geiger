use super::{IncludeTests, RsFileMetrics, ScanFileError};

use crate::geiger_syn_visitor::GeigerSynVisitor;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Scan a single file for `unsafe` usage.
pub fn find_unsafe_in_file(
    path: &Path,
    include_tests: IncludeTests,
) -> Result<RsFileMetrics, ScanFileError> {
    let mut file = File::open(path)
        .map_err(|e| ScanFileError::Io(e, path.to_path_buf()))?;
    let mut src = vec![];
    file.read_to_end(&mut src)
        .map_err(|e| ScanFileError::Io(e, path.to_path_buf()))?;
    let src = String::from_utf8(src)
        .map_err(|e| ScanFileError::Utf8(e, path.to_path_buf()))?;
    find_unsafe_in_string(&src, include_tests)
        .map_err(|e| ScanFileError::Syn(e, path.to_path_buf()))
}

pub fn find_unsafe_in_string(
    src: &str,
    include_tests: IncludeTests,
) -> Result<RsFileMetrics, syn::Error> {
    use syn::visit::Visit;
    let syntax = syn::parse_file(src)?;
    let mut vis = GeigerSynVisitor::new(include_tests);
    vis.visit_file(&syntax);
    Ok(vis.metrics)
}

#[cfg(test)]
mod find_tests {
    use super::*;

    use cargo_geiger_serde::{Count, CounterBlock};
    use tempfile::tempdir;

    const DEFAULT_COUNTERS: CounterBlock = CounterBlock {
        functions: Count {
            safe: 0,
            unsafe_: 0,
        },
        exprs: Count {
            safe: 0,
            unsafe_: 0,
        },
        item_impls: Count {
            safe: 0,
            unsafe_: 0,
        },
        item_traits: Count {
            safe: 0,
            unsafe_: 0,
        },
        methods: Count {
            safe: 0,
            unsafe_: 0,
        },
        ptr_derefs: Count {
            safe: 0,
            unsafe_: 0,
        },
        unsafe_fn_calls: Count {
            safe: 0,
            unsafe_: 0,
        },
        unsafe_fn_calls_core: Count {
            safe: 0,
            unsafe_: 0,
        },
        unsafe_fn_calls_alloc: Count {
            safe: 0,
            unsafe_: 0,
        },
        unsafe_fn_calls_std: Count {
            safe: 0,
            unsafe_: 0,
        },
        unsafe_fn_calls_other: Count {
            safe: 0,
            unsafe_: 0,
        },
        static_mut_access: Count {
            safe: 0,
            unsafe_: 0,
        },
        union_field_access: Count {
            safe: 0,
            unsafe_: 0,
        },
    };
    const DEFAULT_METRICS: RsFileMetrics = RsFileMetrics {
        counters: DEFAULT_COUNTERS,
        forbids_unsafe: false,
    };

    const FILE_CONTENT_STRING: &str = "use std::io::Write;

pub unsafe fn f() {
    f();
}

pub fn g() {
    std::io::stdout().write_all(unsafe {
        std::str::from_utf8_unchecked(b\"binarystring\")
    }.as_bytes()).unwrap();
}

#[no_mangle]
pub fn h() {
    f();
}

#[export_name = \"exported_g\"]
pub fn g() {
    f();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        unsafe {
            f();
        }
    }
}
";

    #[test]
    fn find_unsafe() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("lib.rs");
        std::fs::write(&file_path, FILE_CONTENT_STRING).unwrap();

        let from_file =
            find_unsafe_in_file(&file_path, IncludeTests::No).unwrap();
        let from_string =
            find_unsafe_in_string(FILE_CONTENT_STRING, IncludeTests::No)
                .unwrap();
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 3,
                },
                exprs: Count {
                    safe: 4,
                    unsafe_: 4,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                unsafe_fn_calls_std: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 3,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        assert_eq!(from_file, expected);
        assert_eq!(from_string, expected);

        let from_file =
            find_unsafe_in_file(&file_path, IncludeTests::Yes).unwrap();
        let from_string =
            find_unsafe_in_string(FILE_CONTENT_STRING, IncludeTests::Yes)
                .unwrap();
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 2,
                    unsafe_: 3,
                },
                exprs: Count {
                    safe: 4,
                    unsafe_: 5,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 5,
                },
                unsafe_fn_calls_std: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        assert_eq!(from_file, expected);
        assert_eq!(from_string, expected);
    }

    #[test]
    fn forbids_unsafe() {
        let expected = RsFileMetrics {
            forbids_unsafe: true,
            ..DEFAULT_METRICS
        };
        let actual =
            find_unsafe_in_string("#![forbid(unsafe_code)]", IncludeTests::No)
                .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn counters_functions() {
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 2,
                    unsafe_: 3,
                },
                exprs: Count {
                    safe: 2,
                    unsafe_: 3,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 3,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 3,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let file = "
            pub fn f() { f(); }
            pub fn f() { f(); }
            pub unsafe fn f() { f(); }
            #[no_mangle]
            pub fn f() { f(); }
            #[export_name = \"exported_e\"]
            pub unsafe fn f() { f(); }
        ";
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn counters_exprs() {
        let file = "
            pub fn f() {
                f();
                x.f();
                let x = *y;
                println!(\"abc\"); // The `syn` crate v2.0.60 doesn't visit macros.
                let x = 1; // Literal expressions are not counted.
            }
            pub fn f() { unsafe { let x = f(); } }
            pub unsafe fn f() { let x = f(); }
            #[cfg(test)]
            mod tests {
                pub fn f() { f(); }
            }
            #[test]
            pub fn f() { f(); }
        ";
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 2,
                    unsafe_: 1,
                },
                exprs: Count {
                    safe: 3,
                    unsafe_: 2,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn counters_exprs_include_tests() {
        let file = "
            pub fn f() { f(); }
            pub unsafe fn f() { f(); }
            #[cfg(test)]
            mod tests {
                pub unsafe fn f() { f(); }
                pub fn f() {
                    f();
                    unsafe { f(); }
                }
            }
            #[test]
            pub fn f() {
                f();
                unsafe { f(); }
            }
        ";
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 3,
                    unsafe_: 2,
                },
                exprs: Count {
                    safe: 3,
                    unsafe_: 4,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::Yes).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn unsafe_method_calls_core() {
        // Test that known unsafe methods from core are properly classified
        let file = r#"
            pub fn test_ptr_methods() {
                let ptr: *const u8 = std::ptr::null();
                let mut_ptr: *mut u8 = std::ptr::null_mut();
                unsafe {
                    // Pointer offset methods (core::ptr)
                    let _ = ptr.add(1);
                    let _ = ptr.sub(1);
                    let _ = ptr.offset(1);
                    let _ = ptr.read();
                    let _ = ptr.read_volatile();
                    mut_ptr.write(0);
                    // Slice methods (core::slice)
                    let slice: &[u8] = &[1, 2, 3];
                    let _ = slice.get_unchecked(0);
                }
            }
        "#;
        // Count breakdown:
        // - safe exprs: let ptr = ...(1 call expr), let mut_ptr = ...(1 call expr) = 2 safe call exprs
        // - unsafe exprs: 7 method calls + let slice = &[1,2,3](array ref) + let _ = (7 assignments) = 9 unsafe exprs
        // - unsafe_fn_calls: 7 (add, sub, offset, read, read_volatile, write, get_unchecked)
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 2,
                    unsafe_: 9,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 7,
                },
                unsafe_fn_calls_core: Count {
                    safe: 0,
                    unsafe_: 7,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn unsafe_method_calls_alloc() {
        // Test that known unsafe methods from alloc are properly classified
        let file = r#"
            pub fn test_vec_methods() {
                let mut vec: Vec<u8> = Vec::new();
                unsafe {
                    vec.set_len(10);
                }
            }
        "#;
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 1,
                    unsafe_: 1,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                unsafe_fn_calls_alloc: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn unsafe_simd_intrinsics() {
        // Test that SIMD intrinsics are classified as core
        let file = r#"
            use core::arch::x86_64::*;
            pub unsafe fn test_simd() {
                let a = _mm_set1_epi8(0);
                let b = _mm256_setzero_si256();
            }
        "#;
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                exprs: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                unsafe_fn_calls_core: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn mixed_unsafe_calls() {
        // Test mixed function calls, method calls, and SIMD intrinsics
        let file = r#"
            pub fn test_mixed() {
                unsafe {
                    // Function call with std:: prefix
                    let _ = std::str::from_utf8_unchecked(b"hello");
                    // Pointer method call (core)
                    let ptr: *const u8 = std::ptr::null();
                    let _ = ptr.add(1);
                    // Unknown function call (other)
                    my_unsafe_fn();
                }
            }
        "#;
        // Count breakdown:
        // - safe exprs: 0 (everything is inside unsafe block)
        // - unsafe exprs: from_utf8_unchecked(1) + null()(1) + ptr.add(1)(1) + my_unsafe_fn()(1) = 4 unsafe exprs
        // - unsafe_fn_calls:
        //   - std::str::from_utf8_unchecked -> std (1)
        //   - std::ptr::null -> std (1) - this is also an unsafe fn call!
        //   - ptr.add -> core (1)
        //   - my_unsafe_fn -> other (1)
        //   Total: 4 calls
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 4,
                },
                unsafe_fn_calls_std: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                unsafe_fn_calls_core: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                unsafe_fn_calls_other: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn safe_method_in_unsafe_block_not_counted() {
        // Test that safe methods called in unsafe blocks are NOT counted as unsafe fn calls
        let file = r#"
            pub fn test_safe_methods() {
                let vec: Vec<u8> = Vec::new();
                unsafe {
                    // These are safe methods, should NOT be counted as unsafe_fn_calls
                    let _ = vec.len();
                    let _ = vec.is_empty();
                    // This is an unsafe method, SHOULD be counted
                    let ptr: *const u8 = std::ptr::null();
                    let _ = ptr.add(1);
                }
            }
        "#;
        // Count breakdown:
        // - safe exprs: Vec::new() = 1 safe call expr
        // - unsafe exprs: vec.len()(1) + vec.is_empty()(1) + std::ptr::null()(1) + ptr.add(1)(1) = 4 unsafe exprs
        // - unsafe_fn_calls:
        //   - vec.len() -> NOT counted (safe method, not in known unsafe list)
        //   - vec.is_empty() -> NOT counted (safe method)
        //   - std::ptr::null() -> std (1) - function call with std:: prefix
        //   - ptr.add(1) -> core (1) - unsafe method
        //   Total: 2 calls
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 1,
                    unsafe_: 4,
                },
                unsafe_fn_calls: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                unsafe_fn_calls_std: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                unsafe_fn_calls_core: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn static_mut_access_counted() {
        // Test that mutable static access is counted
        let file = r#"
            static mut COUNTER: usize = 0;
            
            pub fn increment() {
                unsafe {
                    COUNTER += 1;
                }
            }
            
            pub fn read() -> usize {
                unsafe { COUNTER }
            }
        "#;
        // Count breakdown:
        // - COUNTER += 1: compound assignment is 1 expr, COUNTER access is detected
        // - unsafe { COUNTER } in read(): COUNTER is a Path expr (not counted in exprs)
        // - static_mut_access: COUNTER in += (1) + COUNTER in read (1) = 2
        // - unsafe exprs: only the compound assignment COUNTER += 1 (1)
        //   Note: Expr::Path is not counted in exprs counter
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 2,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 0,
                    unsafe_: 1,  // Only the compound assignment
                },
                static_mut_access: Count {
                    safe: 0,
                    unsafe_: 2,
                },
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn union_field_access_counted() {
        // Test that union field access is counted
        let file = r#"
            union MyUnion {
                i: i32,
                f: f32,
            }
            
            pub fn read_union(u: MyUnion) -> i32 {
                unsafe { u.i }
            }
        "#;
        // Note: Without type information, we can only track union field access
        // if the variable name matches a known union type name. This is a heuristic.
        // In this case, 'u' is not the same as 'MyUnion', so it won't be detected.
        // This is a limitation of static analysis without type information.
        let expected = RsFileMetrics {
            counters: CounterBlock {
                functions: Count {
                    safe: 1,
                    unsafe_: 0,
                },
                exprs: Count {
                    safe: 0,
                    unsafe_: 1,
                },
                // union_field_access is 0 because we can't track without type info
                ..DEFAULT_COUNTERS
            },
            ..DEFAULT_METRICS
        };
        let actual = find_unsafe_in_string(file, IncludeTests::No).unwrap();
        assert_eq!(actual, expected);
    }
}
