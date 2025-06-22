//@ run-pass
//! Test that users are able to use stable mir APIs to retrieve
//! discriminant value and type for AdtDef and Coroutine variants

//@ ignore-stage1
//@ ignore-cross-compile
//@ ignore-remote
//@ edition: 2024

#![feature(rustc_private)]
#![feature(assert_matches)]

extern crate rustc_middle;
#[macro_use]
extern crate rustc_smir;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate stable_mir;

use std::io::Write;
use std::ops::ControlFlow;

use stable_mir::CrateItem;
use stable_mir::crate_def::CrateDef;
use stable_mir::mir::{AggregateKind, Rvalue, Statement, StatementKind};
use stable_mir::ty::{IntTy, RigidTy, Ty, TyKind};

const CRATE_NAME: &str = "crate_variant_ty";

/// Test if we can retrieve discriminant info for different types.
fn test_def_tys() -> ControlFlow<()> {
    check_adt_mono();
    check_adt_poly();
    check_adt_poly2();
    check_coroutine_discriminants();

    ControlFlow::Continue(())
}

fn check_adt_mono() {
    let mono = get_fn("mono").expect_body();

    check_statement_is_aggregate_assign(
        &mono.blocks[0].statements[0],
        0,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &mono.blocks[1].statements[0],
        1,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &mono.blocks[2].statements[0],
        2,
        RigidTy::Int(IntTy::Isize),
    );
}

fn check_adt_poly() {
    let poly = get_fn("poly").expect_body();

    check_statement_is_aggregate_assign(
        &poly.blocks[0].statements[0],
        0,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &poly.blocks[1].statements[0],
        1,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &poly.blocks[2].statements[0],
        2,
        RigidTy::Int(IntTy::Isize),
    );
}

fn check_adt_poly2() {
    let poly = get_fn("poly2").expect_body();

    check_statement_is_aggregate_assign(
        &poly.blocks[0].statements[0],
        0,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &poly.blocks[1].statements[0],
        1,
        RigidTy::Int(IntTy::Isize),
    );
    check_statement_is_aggregate_assign(
        &poly.blocks[2].statements[0],
        2,
        RigidTy::Int(IntTy::Isize),
    );
}

fn check_coroutine_discriminants() {
    let crate_items = stable_mir::all_local_items();
    if let Some((def, args)) = crate_items.iter().find_map(|item| {
        let item_ty = item.ty();
        if let TyKind::RigidTy(RigidTy::Coroutine(def, args, ..)) = item_ty.kind() {
            if def.0.name() == "uwu::{closure#0}".to_string() {
                Some((def, args))
            } else {
                None
            }
        } else {
            None
        }
    }) {
        let discrs = def.discriminants(&args);
        assert_eq!(5, discrs.len());
    } else {
        panic!("Cannot find `uwu::{{closure#0}}`. All local items are: {:#?}", crate_items);
    }
}

fn get_fn(name: &str) -> CrateItem {
    stable_mir::all_local_items().into_iter().find(|it| it.name().eq(name)).unwrap()
}

fn check_statement_is_aggregate_assign(
    statement: &Statement,
    expected_discr_val: u128,
    expected_discr_ty: RigidTy,
) {
    if let Statement { kind: StatementKind::Assign(_, rvalue), .. } = statement
        && let Rvalue::Aggregate(aggregate, _) = rvalue
        && let AggregateKind::Adt(adt_def, variant_idx, ..) = aggregate
    {
        let discr = adt_def.discriminant_for_variant(*variant_idx);

        assert_eq!(discr.val, expected_discr_val);
        assert_eq!(discr.ty, Ty::from_rigid_kind(expected_discr_ty));
    } else {
        unreachable!("Unexpected statement");
    }
}

/// This test will generate and analyze a dummy crate using the stable mir.
/// For that, it will first write the dummy crate into a file.
/// Then it will create a `StableMir` using custom arguments and then
/// it will run the compiler.
fn main() {
    let path = "defs_ty_input.rs";
    generate_input(&path).unwrap();
    let args = &[
        "rustc".to_string(),
        "-Cpanic=abort".to_string(),
        "--edition".to_string(),
        "2024".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        path.to_string(),
    ];
    run!(args, test_def_tys).unwrap();
}

fn generate_input(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write!(
        file,
        r#"
        use std::hint::black_box;

        enum Mono {{
            A,
            B(i32),
            C {{ a: i32, b: u32 }},
        }}

        enum Poly<T> {{
            A,
            B(T),
            C {{ t: T }},
        }}

        pub fn main() {{
            mono();
            poly();
            poly2::<i32>(1);
        }}

        fn mono() {{
            black_box(Mono::A);
            black_box(Mono::B(6));
            black_box(Mono::C {{a: 1, b: 10 }});
        }}

        fn poly() {{
            black_box(Poly::<i32>::A);
            black_box(Poly::B(1i32));
            black_box(Poly::C {{ t: 1i32 }});
        }}

        fn poly2<T: Copy>(t: T) {{
            black_box(Poly::<T>::A);
            black_box(Poly::B(t));
            black_box(Poly::C {{ t: t }});
        }}

        async fn uwu() -> i32 {{
            let a = async {{ 1 }}.await;
            let b = async {{ 2 }}.await;
            a + b
        }}
    "#
    )?;
    Ok(())
}
