use swc_experimental_ecma_ast::{Ast, Ident};
use swc_experimental_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_experimental_ecma_visit::{Visit, VisitWith};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringInput::new(source, Default::default(), Default::default());

    let mut parser = Parser::new(syntax, input, None);
    let program = parser.parse_program().unwrap();

    let mut counter = Counter { count: 0 };
    program.visit_with(&mut counter, &parser.ast);
}

struct Counter {
    count: usize,
}

impl Visit for Counter {
    fn visit_ident(&mut self, _node: Ident, _ast: &Ast) {
        self.count += 1;
    }
}
