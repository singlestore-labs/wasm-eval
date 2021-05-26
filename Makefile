REGISTRY=psy3.memcompute.com
BUILDER=${REGISTRY}/wasmer

test:
	cargo test --verbose

run:
	docker run -it \
		-w /wasm/wasm-bundler \
		-v $(shell dirname ${PWD}):/wasm \
		psy3.memcompute.com/wasmer \
		cargo run
