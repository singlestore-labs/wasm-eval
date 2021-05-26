run:
	docker run -it \
		-w /src/copy \
		-v $(shell dirname ${PWD}):/src \
		psy3.memcompute.com/launch-pad/wasmtime \
		rustup target add wasm32-wasi && \
		echo "hello world" > test.txt && \
		wasmtime --dir=. ./target/wasm32-wasi/debug/copy.wasm test.txt test2.txt

exec:
	docker run -it \
		-w /src/copy \
		-v $(shell dirname ${PWD}):/src \
		psy3.memcompute.com/launch-pad/wasmtime sh
