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

use crate::{OriginKind, UnsafeCallRecord};

/// Collects unsafe function calls from HIR
pub struct UnsafeCallCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    records: Vec<UnsafeCallRecord>,
    /// Depth of nested unsafe contexts (unsafe blocks, unsafe functions)
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

        // Get source location
        let source_map = self.tcx.sess.source_map();
        let lo = expr.span.lo();
        let pos = source_map.lookup_char_pos(lo);

        let current_crate = self
            .tcx
            .crate_name(rustc_hir::def_id::LOCAL_CRATE)
            .to_string();

        self.records.push(UnsafeCallRecord {
            crate_name: current_crate,
            file: pos.file.name.prefer_local().to_string(),
            line: pos.line as u32,
            column: pos.col.0 as u32,
            callee_full_path: full_path,
            callee_crate: crate_name,
            origin_kind,
        });
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

        // Process calls in unsafe contexts
        if self.in_unsafe_context() {
            match &expr.kind {
                hir::ExprKind::Call(func, _) => {
                    self.process_function_call(expr, func);
                }
                hir::ExprKind::MethodCall(_, _, _, _) => {
                    self.process_method_call(expr);
                }
                _ => {}
            }
        }

        intravisit::walk_expr(self, expr);
    }
}
