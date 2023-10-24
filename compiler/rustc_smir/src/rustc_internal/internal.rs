//! Module containing the translation from stable mir constructs to the rustc counterpart.
//!
//! This module will only include a few constructs to allow users to invoke internal rustc APIs
//! due to incomplete stable coverage.

// Prefer importing stable_mir over internal rustc constructs to make this file more readable.
use crate::rustc_smir::Tables;
use rustc_middle::ty::{self as rustc_ty, Ty as InternalTy};
use stable_mir::ty::{Const, GenericArgKind, GenericArgs, Region, Ty};
use stable_mir::DefId;

use super::RustcInternal;

impl<'tcx> RustcInternal<'tcx> for DefId {
    type T = rustc_span::def_id::DefId;
    fn internal(&self, tables: &mut Tables<'tcx>) -> Self::T {
        tables.def_ids[*self]
    }
}

impl<'tcx> RustcInternal<'tcx> for GenericArgs {
    type T = rustc_ty::GenericArgsRef<'tcx>;
    fn internal(&self, tables: &mut Tables<'tcx>) -> Self::T {
        tables.tcx.mk_args_from_iter(self.0.iter().map(|arg| arg.internal(tables)))
    }
}

impl<'tcx> RustcInternal<'tcx> for GenericArgKind {
    type T = rustc_ty::GenericArg<'tcx>;
    fn internal(&self, tables: &mut Tables<'tcx>) -> Self::T {
        match self {
            GenericArgKind::Lifetime(reg) => reg.internal(tables).into(),
            GenericArgKind::Type(ty) => ty.internal(tables).into(),
            GenericArgKind::Const(cnst) => ty_const(cnst, tables).into(),
        }
    }
}

impl<'tcx> RustcInternal<'tcx> for Region {
    type T = rustc_ty::Region<'tcx>;
    fn internal(&self, _tables: &mut Tables<'tcx>) -> Self::T {
        todo!()
    }
}

impl<'tcx> RustcInternal<'tcx> for Ty {
    type T = InternalTy<'tcx>;
    fn internal(&self, tables: &mut Tables<'tcx>) -> Self::T {
        tables.types[self.0]
    }
}

fn ty_const<'tcx>(constant: &Const, tables: &mut Tables<'tcx>) -> rustc_ty::Const<'tcx> {
    match constant.internal(tables) {
        rustc_middle::mir::Const::Ty(c) => c,
        cnst => {
            panic!("Trying to covert constant `{constant:?}` to type constant, but found {cnst:?}")
        }
    }
}

impl<'tcx> RustcInternal<'tcx> for Const {
    type T = rustc_middle::mir::Const<'tcx>;
    fn internal(&self, tables: &mut Tables<'tcx>) -> Self::T {
        tables.constants[self.id]
    }
}
