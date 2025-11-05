use crate::{generator::ast_builder::ast_builder, parse::parse_files};

pub(crate) mod generator;
pub(crate) mod output;
pub(crate) mod parse;
pub(crate) mod schema;

const SOURCE_PATHS: &[&str] = &[
    "crates/swc_ecma_ast/src/ast/class.rs",
    "crates/swc_ecma_ast/src/ast/decl.rs",
    "crates/swc_ecma_ast/src/ast/expr.rs",
    "crates/swc_ecma_ast/src/ast/function.rs",
    "crates/swc_ecma_ast/src/ast/ident.rs",
    "crates/swc_ecma_ast/src/ast/jsx.rs",
    "crates/swc_ecma_ast/src/ast/lit.rs",
    "crates/swc_ecma_ast/src/ast/module_decl.rs",
    "crates/swc_ecma_ast/src/ast/module.rs",
    "crates/swc_ecma_ast/src/ast/operator.rs",
    "crates/swc_ecma_ast/src/ast/pat.rs",
    "crates/swc_ecma_ast/src/ast/prop.rs",
    "crates/swc_ecma_ast/src/ast/stmt.rs",
    "crates/swc_ecma_ast/src/ast/typescript.rs",
];

fn main() {
    let schema = parse_files(SOURCE_PATHS);

    let ast_builder_ret = ast_builder(&schema);
    ast_builder_ret.write_to_file().unwrap();
}
