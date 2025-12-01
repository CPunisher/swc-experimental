use colored::Colorize;
use swc_common::comments::SingleThreadedComments;
use swc_experimental_ecma_ast::Program;
use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};
use swc_experimental_ecma_semantic::resolver::resolver;

use crate::{AppArgs, cases::Case, suite::TestResult};

pub struct SemanticRunner;

impl SemanticRunner {
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
                "js" => parser.parse_program(),
                "cjs" => parser
                    .parse_script()
                    .map(|ret| ret.map_root(Program::Script)),
                "mjs" => parser
                    .parse_module()
                    .map(|ret| ret.map_root(Program::Module)),
                "ts" | "jsx" | "tsx" => {
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

            let _semantic = resolver(ret.root, &ret.ast);

            results.push(TestResult::Passed {
                path: case.path().to_owned(),
            });
        }
        results
    }
}
