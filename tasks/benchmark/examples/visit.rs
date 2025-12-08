use swc_core::common::BytePos;
use swc_experimental_ecma_ast::NodeKind;

fn test_legacy(src: &str) -> usize {
    use swc_core::ecma::parser::{Parser, StringInput, Syntax, lexer::Lexer};
    use swc_core::ecma::visit::VisitWith;

    let input = StringInput::new(src, BytePos(0), BytePos(src.len() as u32));
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        input,
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();

    struct Counter {
        count: usize,
    }

    impl swc_core::ecma::visit::Visit for Counter {
        fn visit_ident(&mut self, _node: &swc_core::ecma::ast::Ident) {
            self.count += 1;
        }
    }

    let mut counter = Counter { count: 0 };
    module.visit_with(&mut counter);
    counter.count
}

fn test_new(src: &str) -> usize {
    use swc_experimental_ecma_ast::Ast;
    use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};
    use swc_experimental_ecma_visit::VisitWith;

    let input = StringSource::new(src);
    let lexer = Lexer::new(
        swc_experimental_ecma_parser::Syntax::Es(Default::default()),
        Default::default(),
        input,
        None,
    );
    let parser = Parser::new_from(lexer);
    let ret = parser.parse_module().unwrap();

    struct Counter {
        count: usize,
    }

    impl swc_experimental_ecma_visit::Visit for Counter {
        fn visit_ident(&mut self, _node: swc_experimental_ecma_ast::Ident, _ast: &Ast) {
            self.count += 1;
        }
    }

    let mut counter = Counter { count: 0 };
    ret.root.visit_with(&mut counter, &ret.ast);
    counter.count
}

fn test_post_order(src: &str) -> usize {
    use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};

    let input = StringSource::new(src);
    let lexer = Lexer::new(
        swc_experimental_ecma_parser::Syntax::Es(Default::default()),
        Default::default(),
        input,
        None,
    );
    let parser = Parser::new_from(lexer);
    let ret = parser.parse_module().unwrap();

    let mut counter = 0;
    for (_, node) in ret.ast.nodes() {
        if node.kind() == NodeKind::Ident {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let source = include_str!("../files/typescript.js");
    let legacy = test_legacy(source);
    let new = test_new(source);
    let post_order = test_post_order(source);

    assert_eq!(legacy, new);
    assert_eq!(legacy, post_order);
}
