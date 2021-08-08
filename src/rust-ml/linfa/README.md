# WASM from Linfa

The idea was to use a Rust ML library as Rust supports wasm as a target.

This does not work today. Linfa algorithms depend on bolas and gfortran that do not have wasm safe equivalents.

See <https://github.com/rust-ml/linfa>.
