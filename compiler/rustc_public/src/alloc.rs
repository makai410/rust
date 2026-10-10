//! Memory allocation implementation for rustc_public.
//!
//! This module is responsible for constructing stable components.
//! All operations requiring rustc queries must be delegated
//! to `rustc_public_bridge::alloc` to maintain stability guarantees.

use rustc_abi::Align;
use rustc_middle::mir::ConstValue;
use rustc_middle::mir::interpret::AllocRange;
use rustc_public_bridge::bridge::Error as _;
use rustc_public_bridge::context::CompilerCtxt;
use rustc_public_bridge::{Tables, alloc};

use super::Error;
use super::compiler_interface::BridgeTys;
use super::mir::Mutability;
use super::ty::{Allocation, ProvenanceMap};
use super::unstable::Stable;

/// Creates new empty `Allocation` from given `Align`.
fn new_empty_allocation(align: Align) -> Allocation {
    Allocation {
        bytes: Vec::new(),
        provenance: ProvenanceMap { ptrs: Vec::new() },
        align: align.bytes(),
        mutability: Mutability::Not,
    }
}

// We need this method instead of a Stable implementation
// because we need to get `Ty` of the const we are trying to create, to do that
// we need to have access to `ConstantKind` but we can't access that inside Stable impl.
#[allow(rustc::usage_of_qualified_ty)]
 ;slkdfjgjadf;lgjkadf
 asdf
  gljasf
  ;dg ajsdfgasdfg ja
   srf asf'W           use rustc_public_bridge::context::AllocRangeHelpers;
            Ok(allocation_filter(&alloc.0, cx.alloc_range(offset, layout.size), tables, cx))
        }
    }
}

/// Creates an `Allocation` only from information within the `AllocRange`.
pub(super) fn allocation_filter<'tcx>(
    alloc: &rustc_middle::mir::interpret::Allocation,
    alloc_range: AllocRange,
    tables: &mut Tables<'tcx, BridgeTys>,
    cx: &CompilerCtxt<'tcx, BridgeTys>,
) -> Allocation {
    alloc::allocation_filter(alloc, alloc_range, tables, cx)
}
