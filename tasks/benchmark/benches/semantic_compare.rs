#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn bench_semantic_old(c: &mut Criterion) {
    use swc_core::common::BytePos;
    use swc_core::common::input::SourceFileInput;
    use swc_core::common::{GLOBALS, Globals};
    use swc_core::ecma::parser::{Lexer, Parser};
    use swc_core::ecma::visit::VisitMut;

    let bench_cases = &[("typescript", include_str!("../files/typescript.js"))];
    let mut group = c.benchmark_group("semantic");
    for (name, source) in bench_cases {
        let globals = Globals::default();
        GLOBALS.set(&globals, || {
            group.bench_function(format!("{name}/semantic/legacy"), |b| {
                b.iter_batched(
                    || {
                        let input =
                            SourceFileInput::new(source, BytePos(0), BytePos(source.len() as u32));
                        let lexer = Lexer::new(
                            swc_core::ecma::parser::Syntax::Es(Default::default()),
                            Default::default(),
                            input,
                            None,
                        );
                        let mut parser = Parser::new_from(lexer);
                        parser.parse_module().unwrap()
                    },
                    |mut ret| {
                        let mut resolver = swc_core::ecma::transforms::base::resolver(
                            swc_core::common::Mark::new(),
                            swc_core::common::Mark::new(),
                            false,
                        );
                        resolver.visit_mut_module(&mut ret);
                    },
                    criterion::BatchSize::PerIteration,
                );
            });
        });
    }
}

fn bench_semantic_new(c: &mut Criterion) {
    use swc_experimental_ecma_parser::{Lexer, Parser, StringSource};
    use swc_experimental_ecma_semantic::resolver::resolver;

    let bench_cases = &[("typescript", include_str!("../files/typescript.js"))];
    let mut group = c.benchmark_group("semantic");
    for (name, source) in bench_cases {
        group.bench_function(format!("{name}/semantic/new"), |b| {
            let input = StringSource::new(source);
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

criterion_group!(semantic, bench_semantic_old, bench_semantic_new);
criterion_main!(semantic);
