WASI_ROOT?=/opt/wasi-sdk/bin/

just-clang:
	docker run -it \
		-w /src/c_examples \
		-v $(shell dirname ${PWD}):/src \
		psy3.memcompute.com/launch-pad/wasmtime \
		clang --target=wasm32-unknown-wasi -nostartfiles -Wl,--import-memory -Wl,--no-entry -Wl,--export-all -o fib.wasm fib.c

copy-example:
	docker run -it \
		-w /src/c_examples \
		-v $(shell dirname ${PWD}):/src \
		psy3.memcompute.com/launch-pad/wasmtime \
		${WASI_ROOT}clang --target=wasm32-unknown-wasi -nostartfiles -Wl,--import-memory -Wl,--no-entry -Wl,--export-all -o copy.wasm copy.c && \
		echo "hello world" > test.txt
		wasmtime --dir=. copy.wasm test.txt test2.txt