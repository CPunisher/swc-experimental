use std::collections::HashSet;

use swc_experimental_ecma_ast::{Ast, NodeId};
use swc_experimental_ecma_parser::{EsSyntax, Parser, StringSource, Syntax};
use swc_experimental_ecma_visit::{Visit, VisitWith};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    });
    let input = StringSource::new(source);

    let parser = Parser::new(syntax, input, None);
    let ret = parser.parse_program().unwrap();

    let mut used = Use {
        ast: &ret.ast,
        used: Default::default(),
    };
    ret.root.visit_with(&mut used);

    for (node_id, node) in ret.ast.nodes() {
        let source = &source[node.span().lo.0 as usize - 1..node.span().hi.0 as usize - 1];
        if !used.used.contains(&node_id) {
            println!("Unused node: {:?}, source: {}", node.kind(), source);
        }
    }

    println!(
        "Total: {}, Used: {}",
        ret.ast.nodes().count(),
        used.used.len()
    )
}

struct Use<'a> {
    ast: &'a Ast,
    used: HashSet<NodeId>,
}

impl Visit for Use<'_> {
    fn ast(&self) -> &Ast {
        self.ast
    }

    fn enter_node(&mut self, node_id: NodeId) {
        self.used.insert(node_id);
    }
}
