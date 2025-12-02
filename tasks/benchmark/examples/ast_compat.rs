use swc_experimental_ecma_parser::{EsSyntax, Parser, StringSource, Syntax};

pub fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringSource::new(source);

    let parser = Parser::new(syntax, input, None);
    let ret = parser.parse_program().unwrap();

    let _legacy = swc_experimental_ecma_ast_compat::transform_program(&ret.ast, ret.root);
}
