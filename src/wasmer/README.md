# wasmer

## C API

<https://wasmerio.github.io/wasmer/c/>

## Rust

<https://docs.wasmer.io/integrations/rust>

Wasmer publishes various Crates:
wasmer: The Wasmer Runtime
Compilers:
wasmer-compiler-singlepass: The Singlepass compiler (fast compilation, normal runtime)
wasmer-compiler-cranelift: The Cranelift compiler (normal compilation, a bit faster runtime)
wasmer-compiler-llvm: The LLVM compiler (slower compilation, super fast runtime)
Engines:
wasmer-engine-jit: The JIT Engine
wasmer-engine-native: The Native Engine
Integrations:
wasmer-wasi: Wasmer's implementation of the WASI standard. This allows you to run Wasm in a POSIX-like environment with a file system and permissions.
wasmer-emscripten: Wasmer's implementation of the Emscripten ABI. This allows you to run Wasm in a less sandboxed way in a 32bit Linux-like environment.
