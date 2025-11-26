//! HIR visitor for collecting unsafe function calls

#![cfg(feature = "rustc_private")]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use rustc_hir as hir;
use rustc_hir::def::Res;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::{self, Visitor};
use rustc_middle::hir::nested_filter;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::{
    OriginKind, PtrDerefRecord, StaticMutAccessRecord, UnionFieldAccessRecord,
    UnsafeCallRecord, UnsafeCallReport,
};

/// Collects unsafe operations from HIR using the Rust compiler's type information
pub struct UnsafeCallCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    report: UnsafeCallReport,
    /// Depth of nested unsafe contexts (unsafe blocks, unsafe functions)
    unsafe_depth: u32,
}

impl<'tcx> UnsafeCallCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        UnsafeCallCollector {
            tcx,
            report: UnsafeCallReport::new(),
            unsafe_depth: 0,
        }
    }

    pub fn into_report(self) -> UnsafeCallReport {
        self.report
    }

    fn in_unsafe_context(&self) -> bool {
        self.unsafe_depth > 0
    }

    /// Get the current crate name
    fn current_crate_name(&self) -> String {
        self.tcx
            .crate_name(rustc_hir::def_id::LOCAL_CRATE)
            .to_string()
    }

    /// Get source location info for an expression
    fn get_source_location(&self, span: Span) -> (String, u32, u32) {
        let source_map = self.tcx.sess.source_map();
        let lo = span.lo();
        let pos = source_map.lookup_char_pos(lo);
        (
            pos.file.name.prefer_local().to_string(),
            pos.line as u32,
            pos.col.0 as u32,
        )
    }

    /// Process a function call expression and record it
    fn process_function_call(
        &mut self,
        expr: &'tcx hir::Expr<'tcx>,
        func: &'tcx hir::Expr<'tcx>,
    ) {
        // For regular function calls, we need to resolve the function path
        if let hir::ExprKind::Path(qpath) = &func.kind {
            let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
            if let Res::Def(_, def_id) = typeck_results.qpath_res(qpath, func.hir_id) {
                self.record_call(expr, def_id);
            }
        }
    }

    /// Process a method call expression and record it
    fn process_method_call(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);

        if let Some(def_id) = typeck_results.type_dependent_def_id(expr.hir_id) {
            self.record_call(expr, def_id);
        }
    }

    /// Record a call to the given def_id
    fn record_call(
        &mut self,
        expr: &'tcx hir::Expr<'tcx>,
        def_id: rustc_hir::def_id::DefId,
    ) {
        // Get the full definition path
        let full_path = self.tcx.def_path_str(def_id);
        let crate_name = self.tcx.crate_name(def_id.krate).to_string();
        let origin_kind = OriginKind::from_crate_name(&crate_name);

        let (file, line, column) = self.get_source_location(expr.span);

        self.report.add_record(UnsafeCallRecord {
            crate_name: self.current_crate_name(),
            file,
            line,
            column,
            callee_full_path: full_path,
            callee_crate: crate_name,
            origin_kind,
        });
    }

    /// Process raw pointer dereference
    fn process_ptr_deref(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        let (file, line, column) = self.get_source_location(expr.span);

        self.report.add_ptr_deref(PtrDerefRecord {
            crate_name: self.current_crate_name(),
            file,
            line,
            column,
        });
    }

    /// Process access to a mutable static variable
    fn process_static_mut_access(
        &mut self,
        expr: &'tcx hir::Expr<'tcx>,
        static_name: String,
    ) {
        let (file, line, column) = self.get_source_location(expr.span);

        self.report.add_static_mut_access(StaticMutAccessRecord {
            crate_name: self.current_crate_name(),
            file,
            line,
            column,
            static_name,
        });
    }

    /// Process union field access
    fn process_union_field_access(
        &mut self,
        expr: &'tcx hir::Expr<'tcx>,
        union_type: String,
        field_name: String,
    ) {
        let (file, line, column) = self.get_source_location(expr.span);

        self.report
            .add_union_field_access(UnionFieldAccessRecord {
                crate_name: self.current_crate_name(),
                file,
                line,
                column,
                union_type,
                field_name,
            });
    }

    /// Check if a dereference is a raw pointer dereference using type info
    fn is_raw_pointer_deref(&self, expr: &'tcx hir::Expr<'tcx>) -> bool {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
        let ty = typeck_results.expr_ty(expr);
        // If the type being dereferenced is a raw pointer (*const T or *mut T)
        ty.is_raw_ptr()
    }

    /// Check if a path expression refers to a mutable static
    fn check_static_mut(&mut self, expr: &'tcx hir::Expr<'tcx>, qpath: &hir::QPath<'tcx>) {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
        if let Res::Def(rustc_hir::def::DefKind::Static { mutability, .. }, def_id) =
            typeck_results.qpath_res(qpath, expr.hir_id)
        {
            if mutability.is_mut() {
                let static_name = self.tcx.def_path_str(def_id);
                self.process_static_mut_access(expr, static_name);
            }
        }
    }

    /// Check if a field access is a union field access
    fn check_union_field_access(
        &mut self,
        expr: &'tcx hir::Expr<'tcx>,
        base: &'tcx hir::Expr<'tcx>,
        field: rustc_span::symbol::Ident,
    ) {
        let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
        let base_ty = typeck_results.expr_ty(base);

        // Check if the base type is a union
        if let rustc_middle::ty::TyKind::Adt(adt_def, _) = base_ty.kind() {
            if adt_def.is_union() {
                let union_type = self.tcx.def_path_str(adt_def.did());
                let field_name = field.to_string();
                self.process_union_field_access(expr, union_type, field_name);
            }
        }
    }
}

impl<'tcx> Visitor<'tcx> for UnsafeCallCollector<'tcx> {
    type NestedFilter = nested_filter::All;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_fn(
        &mut self,
        fk: intravisit::FnKind<'tcx>,
        fd: &'tcx hir::FnDecl<'tcx>,
        b: hir::BodyId,
        _s: Span,
        id: LocalDefId,
    ) {
        // Check if function is unsafe
        let is_unsafe = match fk {
            intravisit::FnKind::ItemFn(
                _,
                _,
                hir::FnHeader {
                    safety: hir::HeaderSafety::Normal(hir::Safety::Unsafe),
                    ..
                },
            )
            | intravisit::FnKind::Method(
                _,
                hir::FnSig {
                    header:
                        hir::FnHeader {
                            safety: hir::HeaderSafety::Normal(hir::Safety::Unsafe),
                            ..
                        },
                    ..
                },
            ) => true,
            _ => false,
        };

        if is_unsafe {
            self.unsafe_depth += 1;
        }

        // Use the default walk_fn to properly visit nested items
        intravisit::walk_fn(self, fk, fd, b, id);

        if is_unsafe {
            self.unsafe_depth -= 1;
        }
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        // Check for unsafe blocks
        if let hir::ExprKind::Block(block, _) = &expr.kind {
            if matches!(block.rules, hir::BlockCheckMode::UnsafeBlock(_)) {
                self.unsafe_depth += 1;
                intravisit::walk_expr(self, expr);
                self.unsafe_depth -= 1;
                return;
            }
        }

        // Process operations in unsafe contexts
        if self.in_unsafe_context() {
            match &expr.kind {
                hir::ExprKind::Call(func, _) => {
                    self.process_function_call(expr, func);
                }
                hir::ExprKind::MethodCall(_, _, _, _) => {
                    self.process_method_call(expr);
                }
                hir::ExprKind::Unary(hir::UnOp::Deref, operand) => {
                    // Check if this is a raw pointer dereference
                    if self.is_raw_pointer_deref(operand) {
                        self.process_ptr_deref(expr);
                    }
                }
                hir::ExprKind::Path(qpath) => {
                    // Check for mutable static access
                    self.check_static_mut(expr, qpath);
                }
                hir::ExprKind::Field(base, field) => {
                    // Check for union field access
                    self.check_union_field_access(expr, base, *field);
                }
                _ => {}
            }
        }

        intravisit::walk_expr(self, expr);
    }
}
