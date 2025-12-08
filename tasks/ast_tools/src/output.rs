use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use proc_macro2::TokenStream;
use syn::{File, parse2};

pub fn output_path(krate: &str, path: &str) -> String {
    format!("crates/{krate}/src/generated/{path}.rs")
}

pub struct RawOutput {
    pub path: String,
    pub content: Vec<u8>,
}

impl RawOutput {
    pub fn write_to_file(&self) -> io::Result<()> {
        if let Ok(existing_data) = fs::read(&self.path)
            && existing_data == self.content
        {
            return Ok(());
        }

        let path = Path::new(&self.path);
        if let Some(path) = path.parent() {
            fs::create_dir_all(path)?;
        }

        fs::write(path, &self.content)
    }
}

pub struct RustOutput {
    pub path: String,
    pub tokens: TokenStream,
}

impl From<RustOutput> for RawOutput {
    fn from(output: RustOutput) -> Self {
        let code = match parse2::<File>(output.tokens.clone()) {
            Ok(file) => rust_fmt(&prettyplease::unparse(&file)),
            Err(_) => output.tokens.to_string(),
        };
        Self {
            path: output.path,
            content: code.into_bytes(),
        }
    }
}

fn rust_fmt(source_text: &str) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run rustfmt");

    let stdin = rustfmt.stdin.as_mut().unwrap();
    stdin.write_all(source_text.as_bytes()).unwrap();
    stdin.flush().unwrap();

    let output = rustfmt.wait_with_output().unwrap();
    if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        let error = String::from_utf8(output.stderr).unwrap();
        panic!("Failed to format rust code:\n{error}")
    }
}
