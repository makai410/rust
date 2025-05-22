//@ run-pass

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

use std::ops::ControlFlow;
use rustc_smir::rustc_internal;

const CRATE_NAME: &str = "ice";

/// This function uses the Stable MIR APIs to get information about the test crate.
fn check_ice() -> ControlFlow<()> {
    let local = stable_mir::local_crate();
    for def in local.fn_defs() {
        let _ = def.body();
    }

    ControlFlow::Continue(())
}

fn main() {
    let path = "ice-alloc.rs";
    let args = &[
        "rustc".to_string(),
        "-Copt-level=1".to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        path.to_string(),
    ];
    run!(args, check_ice).unwrap();
}

fn generate_input(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write!(
        file,
        r#"
        #![allow(dead_code, unused_variables)]
        use std::fmt::Debug;

        pub trait Meow<A: Clone + Debug> {{
            fn foo(&self, a: Option<&A>) -> A;

            fn fzz(&self) -> A {{
                self.foo(None)
            }}
        }}

        fn main() {{
        }}
    "#
    )?;
    Ok(())
}
