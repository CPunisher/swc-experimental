use swc_experimental_ecma_parser::{EsSyntax, Parser, StringSource, Syntax};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringSource::new(source);

    let parser = Parser::new(syntax, input, None);
    let _root = parser.parse_program().unwrap();
}
