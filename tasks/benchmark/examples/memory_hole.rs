use std::collections::HashSet;

use swc_experimental_ecma_ast::NodeId;
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
        used: Default::default(),
    };
    ret.root.visit_with(&mut used, &ret.ast);

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

struct Use {
    used: HashSet<NodeId>,
}

impl Visit for Use {
    fn enter_node(&mut self, node_id: NodeId, _ast: &swc_experimental_ecma_ast::Ast) {
        self.used.insert(node_id);
    }
}
