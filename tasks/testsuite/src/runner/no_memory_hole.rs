use std::collections::HashSet;

use colored::Colorize;
use swc_core::common::comments::SingleThreadedComments;
use swc_experimental_ecma_ast::{Ast, NodeId, Program, Visit};
use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};

use crate::{AppArgs, cases::Case, suite::TestResult};

pub struct NoMemoryHoleRunner;

impl NoMemoryHoleRunner {
    pub fn run<C: Case>(args: &AppArgs, cases: &[C]) -> Vec<TestResult> {
        let mut results = Vec::with_capacity(cases.len());
        for case in cases.iter() {
            if args.debug {
                println!("[{}] {:?}", "Debug".green(), case.relative_path());
            }

            if case.should_fail() {
                continue;
            }

            let syntax = case.syntax();
            let input = StringSource::new(case.code());
            let comments = SingleThreadedComments::default();
            let lexer = Lexer::new(syntax, Default::default(), input, Some(&comments));
            let parser = Parser::new_from(lexer);
            let ret = match case.ext().as_str() {
                "js" | "jsx" => parser.parse_program(),
                "cjs" => parser
                    .parse_script()
                    .map(|ret| ret.map_root(Program::Script)),
                "mjs" => parser
                    .parse_module()
                    .map(|ret| ret.map_root(Program::Module)),
                "ts" | "tsx" => {
                    results.push(TestResult::Ignored {
                        path: case.path().to_owned(),
                    });
                    continue;
                }
                _ => unreachable!(),
            };

            let Ok(ret) = ret else {
                continue;
            };

            let mut use_visitor = Use {
                ast: &ret.ast,
                used: HashSet::new(),
            };

            use_visitor.visit_program(ret.root);

            if use_visitor.used.len() != ret.ast.nodes_len() as usize {
                results.push(TestResult::Failed {
                    path: case.path().to_owned(),
                    error: "Memory hole detected".to_string(),
                });
            } else {
                results.push(TestResult::Passed {
                    path: case.path().to_owned(),
                });
            }
        }
        results
    }
}

struct Use<'a> {
    ast: &'a Ast,
    used: HashSet<NodeId>,
}

impl Visit for Use<'_> {
    fn ast(&self) -> &swc_experimental_ecma_ast::Ast {
        self.ast
    }

    fn enter_node(&mut self, node_id: NodeId) {
        self.used.insert(node_id);
    }
}
