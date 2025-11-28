use colored::Colorize;
use swc_common::comments::SingleThreadedComments;
use swc_experimental_ecma_ast::Program;
use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};

use crate::{AppArgs, cases::Case, suite::TestResult};

pub struct MiscParserRunner;

impl MiscParserRunner {
    pub fn run<C: Case>(args: &AppArgs, cases: &[C]) -> Vec<TestResult> {
        let mut results = Vec::with_capacity(cases.len());
        for case in cases.iter() {
            if args.debug {
                println!("[{}] {:?}", "Debug".green(), case.relative_path());
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

            let errors = match ret {
                Ok(ret) => ret.errors,
                Err(e) => vec![e],
            };
            match (case.should_fail(), !errors.is_empty()) {
                (true, false) => results.push(TestResult::Failed {
                    path: case.relative_path().to_owned(),
                    error: "Expected failure, but parsed successfully".to_string(),
                }),
                (true, true) => results.push(TestResult::Passed {
                    path: case.relative_path().to_owned(),
                }),
                (false, false) => results.push(TestResult::Passed {
                    path: case.relative_path().to_owned(),
                }),
                (false, true) => results.push(TestResult::Failed {
                    path: case.relative_path().to_owned(),
                    error: format!("{:?}", errors),
                }),
            }
        }
        results
    }
}
