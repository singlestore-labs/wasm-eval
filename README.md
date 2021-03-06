# WASM Runtime Evaluator

Curious how to use the various runtimes and what the performance
characteristics for each runtime? Try this comparison tool.

## Evaluation

- Feature differences (e.g. Ref types?)
- Compilation time
- Runtime performance
- Memory usage
- Runs on embeddable devices?
- Run https://github.com/caspervonb/wasi-test-suite

## Test

Suite of tests for primitive types and interacting with memory.

```rust
const PRIMITIVES: &[&str] = &[
    "f64", "f32", "u64", "i64", "u32", "i32", "u16", "i16", "u8", "i8",
];
```

## Benches

<https://00f.net/2021/02/22/webassembly-runtimes-benchmarks/>

> WAVM is based on LLVM. Intuitively, we may expect the LLVM backend of wasmer to perform pretty much the same, but WAVM was still about 15% faster than wasmer.

<https://github.com/wasm3/wasm-coremark>

## Other resources

- <https://webassembly.studio/>
- <https://mbebenita.github.io/WasmExplorer/>


WASM is language independent. This is designed for benchmarks from multiple languages.

Not validating outputs

Improper scoring
(e.g. overweighting microbenchmarks)

Improper workloads
(e.g. time-dependent)

⇒ Measure the wrong thing ⇒ Optimize the wrong thing

Categories of benchmarks:
- Microbenchmarks: synthetic numeric, memory accesses, table operations, mem copy, fill, compile time, instatiate time
- Kernels
- Applications

http://kripken.github.io/Massive/

PolyBenchC (written in C)

http://web.cse.ohio-state.edu/~pouchet.2/software/polybench/

Ostrich Benchmark


Applications: AI/ML focus

