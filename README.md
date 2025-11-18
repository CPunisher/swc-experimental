## SWC (Experimental)

This project is still in a very early stage; please continue to use [SWC](https://github.com/swc-project/swc).

This project is an experimental version of [SWC](https://github.com/swc-project/swc). The primary modification, based on SWC, is the redesign of the AST data structure:

1. It adopts a Flattening AST design, which allows for a higher performance.
2. The AST structure (fields) of the SWC (experimental) remains largely consistent with that of SWC, making migration relatively straightforward.

## Performance

The parser of SWC (experimental), although doesn't support parsing TypeScript and JSX, is 1.2-1.3x faster than SWC. Here's a table of running the command `cargo bench --bench parser` with [9e6fc](https://github.com/CPunisher/swc-experimental/commit/9e6fc).

| Bench Case             | SWC (experimental) | SWC                |
| ---------------------- | ------------------ | ------------------ |
| angular-1.2.5.js       | `2.1 ms` (1.00x)   | `2.7 ms` (1.00x)   |
| backbone-1.1.0.js      | `270.2 us` (1.00x) | `362.5 us` (1.34x) |
| colors.js              | `68.9 ns` (1.00x)  | `63.2 ns` (0.92x)  |
| jquery-1.9.1.js        | `1.6 ms` (1.00x)   | `2.3 ms` (1.44x)   |
| jquery.mobile-1.4.2.js | `2.5 ms` (1.00x)   | `3.2 ms` (1.28x)   |
| moontools-1.4.5.js     | `1.2 ms` (1.00x)   | `1.6 ms` (1.33x)   |
| three-0.138.3.js       | `7.0 ms` (1.00x)   | `9.4 ms` (1.34x)   |
| underscore.js          | `232.4 us` (1.00x) | `310.9 us` (1.34x) |
| yui-3.12.0.js          | `1.2 ms` (1.00x)   | `1.5 ms` (1.25x)   |
| typescript.js          | `43.2 ms` (1.00x)  | `57.3 ms` (1.37x)  |

## Roadmap

There's a long way to go, but luckly it could be progressive.

- [x] Flattening AST design
- [ ] Port tests.
- [ ] Port SWC parser. Currently support parsing JavaScript, but not TypeScript and JSX.
- [ ] Port SWC minifier.
- [ ] Port SWC transformation.

## Credits

Thanks to:

- The [zig](https://github.com/ziglang/zig) compiler and the [rowan](https://github.com/rust-analyzer/rowan), which inspire the flattening AST design.
- The [oxc](https://github.com/oxc-project/oxc), which inspires the ast-tools design.

## License

SWC (Experimental) is licensed under the Apache License, Version 2.0, which should be always same as [SWC](https://github.com/swc-project/swc).

See the [LICENSE](LICENSE) file for more details.
