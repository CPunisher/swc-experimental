use swc_experimental_ecma_parser::{EsSyntax, Parser, StringAllocator, StringSource, Syntax};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringSource::new(source);
    let string_allocator = StringAllocator::new();

    let parser = Parser::new(syntax, input, None, string_allocator);
    let _root = parser.parse_program().unwrap();
}
