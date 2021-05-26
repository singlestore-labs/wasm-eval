//! A Wasm module can export entities, like functions, memories,
//! globals and tables.
//!
//! This example illustrates how to use exported memories
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example exported-memory --release --features "cranelift"
//! ```
//!
//! Ready?

use wasmer::{imports, wat2wasm, Array, Instance, Module, Store, WasmPtr};
use wasmer::Cranelift;
use wasmer::JIT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        br#"
    (module
    (memory (export "mem") 1)
    (global $offset i32 (i32.const 42))
    (global $length (mut i32) (i32.const 13))
    (func (export "load") (result i32 i32)
    global.get $offset
    global.get $length)
    (data (global.get $offset) "Hello, World!"))
    "#,
    )?;

    // let wasm_bytes = std::fs::read("../wasm/wasm_add/wasm_add.wasm")?;

    let engine = JIT::new(Cranelift::default()).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {};

    println!("Instantiating module...");
    let instance = Instance::new(&module, &import_object)?;

    let load = instance
        .exports
        .get_native_function::<(), (WasmPtr<u8, Array>, i32)>("load")?;

    // Memory!
    // The Wasm module exports a memory under "mem". Let's get it.
    let memory = instance.exports.get_memory("mem")?;

    // Now that we have the exported memory, let's get some
    // information about it.
    //
    // The first thing we might be intersted in is the size of the memory.
    // Let's get it!
    println!("Memory size (pages) {:?}", memory.size());
    println!("Memory size (bytes) {:?}", memory.data_size());

    // Next, we'll want to read the contents of the memory.
    //
    // Fortunately, the Wasm module exports a `load` function
    // which will tell us the offset and length of the string.
    let (ptr, length) = load.call()?;
    println!("String offset: {:?}", ptr.offset());
    println!("String length: {:?}", length);

    // We now know where to fin our string, let's read it.
    //
    // We will get bytes out of the memory so we need to
    // decode them into a string.
    let str = ptr.get_utf8_string(memory, length as u32).unwrap();
    println!("Memory contents: {:?}", str);

    // What about changing the contents of the memory with a more
    // appropriate string?
    //
    // To do that, we'll dereference our pointer and change the content
    // of each `Cell`
    let new_str = b"Hello, Wasmer!";
    let values = ptr.deref(memory, 0, new_str.len() as u32).unwrap();
    for i in 0..new_str.len() {
        values[i].set(new_str[i]);
    }

    // And now, let's see the result.
    //
    // Since the new strings is bigger than the older one, we
    // query the length again. The offset remains the same as
    // before.
    println!("New string length: {:?}", new_str.len());

    let str = ptr.get_utf8_string(memory, new_str.len() as u32).unwrap();
    println!("New memory contents: {:?}", str);

    Ok(())
}

#[test]
fn test_exported_memory() -> Result<(), Box<dyn std::error::Error>> {
    main()
}