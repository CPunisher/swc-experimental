use swc_experimental_ecma_ast::{Ast, Ident};
use swc_experimental_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_experimental_ecma_visit::{Visit, VisitWith};

fn main() {
    let source = include_str!("../../swc_ecma_parser/benches/files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringInput::new(source, Default::default(), Default::default());

    let mut parser = Parser::new(syntax, input, None);
    let root = parser.parse_program().unwrap();

    let mut visitor = IdentCount { count: 0 };
    root.visit_children_with(&mut visitor, &parser.ast);
    println!("ident count: {}", visitor.count);
}

struct IdentCount {
    count: u32,
}

impl Visit for IdentCount {
    fn visit_ident(&mut self, _node: Ident, _ast: &Ast) {
        self.count += 1;
    }
}
