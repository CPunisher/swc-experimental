use colored::Colorize;
use swc_core::common::comments::SingleThreadedComments;
use swc_experimental_ecma_ast::{NodeKind, Program};
use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};
use swc_experimental_ecma_transforms_base::remove_paren;

use crate::{AppArgs, cases::Case, suite::TestResult};

pub struct RemoveParenRunner;

impl RemoveParenRunner {
    pub fn run<C: Case>(args: &AppArgs, cases: &[C]) -> Vec<TestResult> {
        let mut results = Vec::with_capacity(cases.len());
        'outer: for case in cases.iter() {
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
            let mut ret = match case.ext().as_str() {
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
            }
            .expect("Failure cases are filtered");

            remove_paren::remove_paren(ret.root, &mut ret.ast, None);
            for (_, node_id) in ret.ast.nodes() {
                if node_id.kind() == NodeKind::ParenExpr {
                    results.push(TestResult::Failed {
                        path: case.relative_path().to_owned(),
                        error: "ParenExpr is detected".to_string(),
                    });
                    continue 'outer;
                }
            }

            results.push(TestResult::Passed {
                path: case.relative_path().to_owned(),
            });
        }
        results
    }
}
