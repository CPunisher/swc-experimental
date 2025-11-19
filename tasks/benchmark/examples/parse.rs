use swc_experimental_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringInput::new(source, Default::default(), Default::default());

    let mut parser = Parser::new(syntax, input, None);
    let _root = parser.parse_program().unwrap();
}
