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


  # sclblmain.c sclblutil.c         # c code
  # -w                              # disable all diagnostics.
  # -O3                             # compiler optimization type 3
  # -fno-exceptions                 # lose the exceptions - they are not supported by wasm
  # -Wl,--export-all                # export all functions
  # -o sclbl.wasm                   # compile to sclbl.wasm
# wasicc sclblmain.c sclblutil.c -w -O3 -fno-exceptions -o sclbl-intsum.wasm \
#  -Wl,--export=pred,--export=malloc_buffer,--export=free_buffer,\
#  --export=get_out_len,--export=get_out_loc