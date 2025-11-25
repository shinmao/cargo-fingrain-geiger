//! Compiler callbacks for rustc integration

#![cfg(feature = "rustc_private")]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

use rustc_driver::Callbacks;
use rustc_hir::intravisit;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use std::path::PathBuf;

use crate::visitor::UnsafeCallCollector;
use crate::UnsafeCallReport;

/// Callback for rustc compilation that collects unsafe calls
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
        _compiler: &Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let mut visitor = UnsafeCallCollector::new(tcx);

        // Visit all items in the crate
        let crate_items = tcx.hir_crate_items(());

        // Visit all free items (functions, statics, consts, etc.)
        for id in crate_items.free_items() {
            let node = tcx.hir_node_by_def_id(id.owner_id.def_id);
            if let rustc_hir::Node::Item(item) = node {
                intravisit::walk_item(&mut visitor, item);
            }
        }

        // Visit all impl items
        for id in crate_items.impl_items() {
            let node = tcx.hir_node_by_def_id(id.owner_id.def_id);
            if let rustc_hir::Node::ImplItem(impl_item) = node {
                intravisit::walk_impl_item(&mut visitor, impl_item);
            }
        }

        // Visit all trait items
        for id in crate_items.trait_items() {
            let node = tcx.hir_node_by_def_id(id.owner_id.def_id);
            if let rustc_hir::Node::TraitItem(trait_item) = node {
                intravisit::walk_trait_item(&mut visitor, trait_item);
            }
        }

        let records = visitor.into_records();
        let report = UnsafeCallReport { records };

        // Write the report to the output file
        if let Err(e) = report.write_to_file(&self.output_path) {
            eprintln!("Failed to write unsafe call report: {}", e);
        }

        // Continue compilation (though we don't need the final artifacts)
        rustc_driver::Compilation::Continue
    }
}
