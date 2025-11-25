use super::{
    file_forbids_unsafe, has_unsafe_attributes, is_test_fn, is_test_mod,
    IncludeTests, RsFileMetrics,
};

use syn::{
    visit, Expr, ExprCall, ExprField, ExprMethodCall, ExprPath, ExprUnsafe,
    ImplItemFn, ItemFn, ItemImpl, ItemMod, ItemStatic, ItemTrait, ItemUnion,
    StaticMutability, UnOp,
};

use std::collections::HashSet;

/// Classification result for unsafe calls/methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsafeCallOrigin {
    Core,
    Alloc,
    Std,
    Other,
}

pub struct GeigerSynVisitor {
    /// Count unsafe usage inside tests
    include_tests: IncludeTests,

    /// The resulting data from a single file scan.
    pub metrics: RsFileMetrics,

    /// The number of nested unsafe scopes that the GeigerSynVisitor are
    /// currently in. For example, if the visitor is inside an unsafe function
    /// and inside an unnecessary unsafe block inside that function, then this
    /// number should be 2. If the visitor is outside unsafe scopes, in a safe
    /// scope, this number should be 0.
    /// This is needed since unsafe scopes can be nested and we need to know
    /// when we leave the outmost unsafe scope and get back into a safe scope.
    unsafe_scopes: u32,

    /// Names of `static mut` variables declared in the current file
    mutable_statics: HashSet<String>,

    /// Names of union types declared in the current file
    union_types: HashSet<String>,
}

impl GeigerSynVisitor {
    pub fn new(include_tests: IncludeTests) -> Self {
        GeigerSynVisitor {
            include_tests,
            metrics: Default::default(),
            unsafe_scopes: 0,
            mutable_statics: HashSet::new(),
            union_types: HashSet::new(),
        }
    }

    pub fn enter_unsafe_scope(&mut self) {
        self.unsafe_scopes += 1;
    }

    pub fn exit_unsafe_scope(&mut self) {
        self.unsafe_scopes -= 1;
    }

    /// Count an unsafe function/method call with its classification
    fn count_unsafe_call(&mut self, origin: UnsafeCallOrigin) {
        // Increment total unsafe_fn_calls
        self.metrics.counters.unsafe_fn_calls.count(true);

        // Classify by origin
        match origin {
            UnsafeCallOrigin::Core => {
                self.metrics.counters.unsafe_fn_calls_core.count(true);
            }
            UnsafeCallOrigin::Alloc => {
                self.metrics.counters.unsafe_fn_calls_alloc.count(true);
            }
            UnsafeCallOrigin::Std => {
                self.metrics.counters.unsafe_fn_calls_std.count(true);
            }
            UnsafeCallOrigin::Other => {
                self.metrics.counters.unsafe_fn_calls_other.count(true);
            }
        }
    }

    /// Classify a function call based on its path
    fn classify_function_call(&self, path: &str) -> UnsafeCallOrigin {
        let path_lower = path.to_lowercase();

        // Check for std:: prefix
        if path_lower.starts_with("std::")
            || path_lower.starts_with("::std::")
            || path_lower.contains("::std::")
        {
            return UnsafeCallOrigin::Std;
        }
        // Check for core:: prefix
        if path_lower.starts_with("core::")
            || path_lower.starts_with("::core::")
            || path_lower.contains("::core::")
        {
            return UnsafeCallOrigin::Core;
        }
        // Check for alloc:: prefix
        if path_lower.starts_with("alloc::")
            || path_lower.starts_with("::alloc::")
            || path_lower.contains("::alloc::")
        {
            return UnsafeCallOrigin::Alloc;
        }

        // Check for common std/core functions called without prefix
        if let Some(origin) = classify_known_unsafe_function(&path_lower) {
            return origin;
        }

        // Everything else is "other"
        UnsafeCallOrigin::Other
    }

    /// Extract function path from an expression
    fn get_call_path(&self, func: &Expr) -> Option<String> {
        match func {
            Expr::Path(ExprPath { path, .. }) => {
                let segments: Vec<String> = path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect();
                Some(segments.join("::"))
            }
            _ => None,
        }
    }
}

/// Known unsafe functions from std/core/alloc that might be called without full path
/// Returns the origin if this is a known unsafe function, None otherwise
fn classify_known_unsafe_function(path: &str) -> Option<UnsafeCallOrigin> {
    // Extract the last segment (function name) from the path
    let func_name = path.rsplit("::").next().unwrap_or(path);

    // Check for module-qualified paths (e.g., "ptr::read", "mem::transmute")
    if path.contains("::") {
        // ptr module functions (core::ptr)
        if path.contains("ptr::") {
            return Some(UnsafeCallOrigin::Core);
        }
        // mem module functions (core::mem)
        if path.contains("mem::") {
            return Some(UnsafeCallOrigin::Core);
        }
        // slice module functions (core::slice)
        if path.contains("slice::") {
            return Some(UnsafeCallOrigin::Core);
        }
        // str module functions (core::str)
        if path.contains("str::") {
            return Some(UnsafeCallOrigin::Core);
        }
        // arch module for SIMD intrinsics (core::arch)
        if path.contains("arch::") {
            return Some(UnsafeCallOrigin::Core);
        }
        // intrinsics module (core::intrinsics)
        if path.contains("intrinsics::") {
            return Some(UnsafeCallOrigin::Core);
        }
    }

    // Check for SIMD intrinsics (called without prefix after `use core::arch::*`)
    if is_simd_intrinsic(func_name) {
        return Some(UnsafeCallOrigin::Core);
    }

    // Check for known unsafe functions by name
    if is_core_unsafe_function(func_name) {
        return Some(UnsafeCallOrigin::Core);
    }

    None
}

/// Check if a function name is a SIMD intrinsic from core::arch
fn is_simd_intrinsic(name: &str) -> bool {
    // x86/x86_64 intrinsics
    if name.starts_with("_mm_")
        || name.starts_with("_mm256_")
        || name.starts_with("_mm512_")
        || name.starts_with("__m")
    {
        return true;
    }

    // ARM NEON intrinsics
    if name.starts_with("v") && (name.contains("q_") || name.ends_with("_s8")
        || name.ends_with("_s16") || name.ends_with("_s32") || name.ends_with("_s64")
        || name.ends_with("_u8") || name.ends_with("_u16") || name.ends_with("_u32")
        || name.ends_with("_u64") || name.ends_with("_f32") || name.ends_with("_f64"))
    {
        return true;
    }

    // Common x86 prefetch/fence instructions
    if name.starts_with("_prefetch")
        || name == "_rdtsc"
        || name == "_rdtscp"
        || name == "_mm_pause"
        || name == "_mm_clflush"
        || name == "_mm_mfence"
        || name == "_mm_sfence"
        || name == "_mm_lfence"
    {
        return true;
    }

    false
}

/// Check if a function name is a known unsafe function from core
fn is_core_unsafe_function(name: &str) -> bool {
    matches!(
        name,
        // ptr module unsafe functions
        "read"
            | "read_unaligned"
            | "read_volatile"
            | "write"
            | "write_unaligned"
            | "write_volatile"
            | "write_bytes"
            | "copy"
            | "copy_nonoverlapping"
            | "swap"
            | "swap_nonoverlapping"
            | "replace"
            | "drop_in_place"
            // mem module unsafe functions
            | "transmute"
            | "transmute_copy"
            | "zeroed"
            | "uninitialized"
            | "size_of_val_raw"
            | "align_of_val_raw"
            // slice module unsafe functions
            | "from_raw_parts"
            | "from_raw_parts_mut"
            // str module unsafe functions
            | "from_utf8_unchecked"
            | "from_utf8_unchecked_mut"
            // intrinsics
            | "unreachable_unchecked"
            | "assume"
    )
}

/// Known unsafe methods from std/core/alloc
/// These are methods that are unsafe when called on certain types
/// Returns the origin if this is a known unsafe method, None otherwise
fn classify_known_unsafe_method(method_name: &str) -> Option<UnsafeCallOrigin> {
    // Raw pointer methods (core::ptr)
    // These are unsafe methods on *const T and *mut T
    let ptr_methods = [
        "add",
        "sub",
        "offset",
        "offset_from",
        "byte_add",
        "byte_sub",
        "byte_offset",
        "read",
        "read_unaligned",
        "read_volatile",
        "write",
        "write_unaligned",
        "write_volatile",
        "write_bytes",
        "copy_to",
        "copy_to_nonoverlapping",
        "copy_from",
        "copy_from_nonoverlapping",
        "drop_in_place",
        "replace",
        "swap",
        "as_ref",       // unsafe on raw pointers
        "as_mut",       // unsafe on raw pointers
        "as_uninit_ref",
        "as_uninit_mut",
        "as_uninit_slice",
        "as_uninit_slice_mut",
    ];

    // Slice methods (core::slice)
    let slice_methods = [
        "get_unchecked",
        "get_unchecked_mut",
        "as_chunks_unchecked",
        "as_chunks_unchecked_mut",
        "split_at_unchecked",
        "split_at_mut_unchecked",
        "align_to",
        "align_to_mut",
    ];

    // String/str methods (alloc::string, core::str)
    let string_methods = [
        "as_bytes_mut",
        "as_mut_vec",  // String::as_mut_vec
        "from_utf8_unchecked",
        "from_utf8_unchecked_mut",
    ];

    // Vec methods (alloc::vec)
    let vec_methods = [
        "set_len",
        "from_raw_parts",
    ];

    // Box methods (alloc::boxed)
    let box_methods = [
        "from_raw",
        "from_raw_in",
    ];

    // CStr/CString methods (core::ffi, alloc::ffi)
    let cstr_methods = [
        "from_ptr",
        "from_bytes_with_nul_unchecked",
    ];

    // NonNull methods (core::ptr)
    let nonnull_methods = [
        "as_ref",
        "as_mut",
        "new_unchecked",
    ];

    // MaybeUninit methods (core::mem)
    let maybeuninit_methods = [
        "assume_init",
        "assume_init_read",
        "assume_init_ref",
        "assume_init_mut",
        "assume_init_drop",
        "array_assume_init",
        "slice_assume_init_ref",
        "slice_assume_init_mut",
    ];

    // Cell/UnsafeCell methods (core::cell)
    let cell_methods = [
        "get",  // UnsafeCell::get returns *mut T, usage can be unsafe
    ];

    // Atomics methods (core::sync::atomic)
    let atomic_methods = [
        "from_ptr",
    ];

    // Check all categories
    if ptr_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if slice_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if maybeuninit_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if nonnull_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if cell_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if atomic_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }
    if string_methods.contains(&method_name) {
        // These could be core or alloc, use Core as they're fundamental
        return Some(UnsafeCallOrigin::Core);
    }
    if vec_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Alloc);
    }
    if box_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Alloc);
    }
    if cstr_methods.contains(&method_name) {
        return Some(UnsafeCallOrigin::Core);
    }

    None
}

impl<'ast> visit::Visit<'ast> for GeigerSynVisitor {
    fn visit_file(&mut self, i: &'ast syn::File) {
        self.metrics.forbids_unsafe = file_forbids_unsafe(i);
        visit::visit_file(self, i);
    }

    /// Free-standing functions
    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        if IncludeTests::No == self.include_tests && is_test_fn(item_fn) {
            return;
        }
        let unsafe_fn =
            item_fn.sig.unsafety.is_some() || has_unsafe_attributes(item_fn);
        if unsafe_fn {
            self.enter_unsafe_scope()
        }
        self.metrics.counters.functions.count(unsafe_fn);
        visit::visit_item_fn(self, item_fn);
        if item_fn.sig.unsafety.is_some() {
            self.exit_unsafe_scope()
        }
    }

    fn visit_expr(&mut self, i: &Expr) {
        // Total number of expressions of any type
        match i {
            Expr::Lit(..) | Expr::Unsafe(..) => {
                // Do not count.
            }
            Expr::Path(expr_path) => {
                // Check for mutable static access
                if self.unsafe_scopes > 0 {
                    // Get the identifier from the path
                    if let Some(ident) = expr_path.path.get_ident() {
                        let name = ident.to_string();
                        if self.mutable_statics.contains(&name) {
                            self.metrics.counters.static_mut_access.count(true);
                        }
                    } else if expr_path.path.segments.len() > 0 {
                        // Check last segment for paths like Self::STATIC_MUT
                        let last_seg = expr_path.path.segments.last().unwrap();
                        let name = last_seg.ident.to_string();
                        if self.mutable_statics.contains(&name) {
                            self.metrics.counters.static_mut_access.count(true);
                        }
                    }
                }
                // Don't count Path as an expression for exprs counter
            }
            Expr::Field(ExprField { base, .. }) => {
                // Check for union field access
                // This is a heuristic: if we're accessing a field in unsafe context,
                // it might be a union field access. Without type info, we can't be certain.
                // We'll count it if the base expression appears to be a union type.
                if self.unsafe_scopes > 0 {
                    // Check if base is a path that matches a known union type
                    if let Expr::Path(expr_path) = base.as_ref() {
                        if let Some(ident) = expr_path.path.get_ident() {
                            let name = ident.to_string();
                            if self.union_types.contains(&name) {
                                self.metrics.counters.union_field_access.count(true);
                            }
                        }
                    }
                }
                self.metrics.counters.exprs.count(self.unsafe_scopes > 0);
            }
            Expr::Unary(unary_expr) => {
                // Check for raw pointer dereference
                if matches!(unary_expr.op, UnOp::Deref(_)) {
                    // If we're inside an unsafe scope, this is likely a raw pointer deref
                    if self.unsafe_scopes > 0 {
                        self.metrics.counters.ptr_derefs.count(true);
                    }
                }
                // Count the expression itself
                self.metrics.counters.exprs.count(self.unsafe_scopes > 0);
            }
            Expr::Call(ExprCall { func, .. }) => {
                // Check for function calls in unsafe context
                if self.unsafe_scopes > 0 {
                    if let Some(path) = self.get_call_path(func) {
                        let origin = self.classify_function_call(&path);
                        self.count_unsafe_call(origin);
                    }
                }
                // Count the expression itself
                self.metrics.counters.exprs.count(self.unsafe_scopes > 0);
            }
            Expr::MethodCall(ExprMethodCall { method, .. }) => {
                // Check for method calls in unsafe context
                if self.unsafe_scopes > 0 {
                    let method_name = method.to_string();
                    // Only count if it's a known unsafe method from std/core/alloc
                    // or count as "other" since we're in an unsafe context
                    if let Some(origin) = classify_known_unsafe_method(&method_name) {
                        self.count_unsafe_call(origin);
                    }
                    // Note: We don't count unknown method calls as unsafe_fn_calls
                    // because they might be safe methods called in an unsafe block.
                    // Without type information, we can't determine if the method is unsafe.
                }
                // Count the expression itself
                self.metrics.counters.exprs.count(self.unsafe_scopes > 0);
            }
            _ => {
                self.metrics.counters.exprs.count(self.unsafe_scopes > 0);
            }
        }
        // This calls `visit_expr_unsafe`.
        visit::visit_expr(self, i);
    }

    fn visit_expr_unsafe(&mut self, i: &ExprUnsafe) {
        self.enter_unsafe_scope();
        visit::visit_expr_unsafe(self, i);
        self.exit_unsafe_scope();
    }

    fn visit_item_mod(&mut self, i: &ItemMod) {
        if IncludeTests::No == self.include_tests && is_test_mod(i) {
            return;
        }
        visit::visit_item_mod(self, i);
    }

    fn visit_item_impl(&mut self, i: &ItemImpl) {
        // unsafe trait impl's
        self.metrics.counters.item_impls.count(i.unsafety.is_some());
        visit::visit_item_impl(self, i);
    }

    fn visit_item_trait(&mut self, i: &ItemTrait) {
        // Unsafe traits
        self.metrics
            .counters
            .item_traits
            .count(i.unsafety.is_some());
        visit::visit_item_trait(self, i);
    }

    fn visit_impl_item_fn(&mut self, i: &ImplItemFn) {
        if i.sig.unsafety.is_some() {
            self.enter_unsafe_scope()
        }
        self.metrics
            .counters
            .methods
            .count(i.sig.unsafety.is_some());
        visit::visit_impl_item_fn(self, i);
        if i.sig.unsafety.is_some() {
            self.exit_unsafe_scope()
        }
    }

    fn visit_item_static(&mut self, i: &ItemStatic) {
        // Track mutable static variables
        if matches!(i.mutability, StaticMutability::Mut(_)) {
            let name = i.ident.to_string();
            self.mutable_statics.insert(name);
        }
        visit::visit_item_static(self, i);
    }

    fn visit_item_union(&mut self, i: &ItemUnion) {
        // Track union types - accessing their fields is unsafe
        let name = i.ident.to_string();
        self.union_types.insert(name);
        visit::visit_item_union(self, i);
    }

    // TODO: Visit macros.
    //
    // TODO: Figure out if there are other visit methods that should be
    // implemented here.
}
