use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::cases::{Case, fixtures};

pub struct Test262ParserCase {
    path: PathBuf,
    code: String,
    should_fail: bool,
}

impl Test262ParserCase {
    pub fn read() -> Vec<Self> {
        let test_path = fixtures().join("test262-parser-tests");
        let pass_cases = read_dir(test_path.join("pass")).unwrap();
        let pass_explicit_cases = read_dir(test_path.join("pass-explicit")).unwrap();
        let fail_cases = read_dir(test_path.join("fail")).unwrap();

        let mut cases = Vec::new();
        for pass_case in pass_cases.chain(pass_explicit_cases) {
            let pass_case = pass_case.unwrap();
            let path = pass_case.path();
            let code = std::fs::read_to_string(&path).unwrap();
            cases.push(Self {
                path,
                code,
                should_fail: false,
            });
        }

        for fail_case in fail_cases {
            let fail_case = fail_case.unwrap();
            let path = fail_case.path();
            let code = std::fs::read_to_string(&path).unwrap();
            cases.push(Self {
                path,
                code,
                should_fail: true,
            });
        }

        cases
    }
}

impl Case for Test262ParserCase {
    fn path(&self) -> &Path {
        &self.path
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn should_fail(&self) -> bool {
        self.should_fail
    }
}
