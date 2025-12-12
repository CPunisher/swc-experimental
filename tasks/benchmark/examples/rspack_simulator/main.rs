use std::env;

pub mod experimental;
pub mod legacy;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    let source = include_str!("../../files/typescript.js");
    let target = env::var("TARGET").unwrap();
    if target == "legacy" {
        legacy::run(source);
    }

    if target == "exp" {
        experimental::run(source, false);
    }

    if target == "exp_compat" {
        experimental::run(source, true);
    }
}
