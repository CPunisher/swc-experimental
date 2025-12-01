use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};
use swc_experimental_ecma_semantic::resolver::resolver;

fn bench_semantic(c: &mut Criterion) {
    let bench_cases = &[("typescript", include_str!("../files/typescript.js"))];
    let mut group = c.benchmark_group("semantic");
    for (name, source) in bench_cases {
        group.bench_function(&format!("{name}/semantic/legacy"), |b| {
            let input = StringSource::new(&source);
            let lexer = Lexer::new(
                swc_experimental_ecma_parser::Syntax::Es(Default::default()),
                Default::default(),
                input,
                None,
            );
            let parser = Parser::new_from(lexer);
            let ret = parser.parse_module().unwrap();
            b.iter(|| {
                black_box(resolver(ret.root, &ret.ast));
            });
        });
    }
}

criterion_group!(semantic, bench_semantic);
criterion_main!(semantic);
