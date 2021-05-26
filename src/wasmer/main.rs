let wasm_bytes = wat2wasm(br#"
(module
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#)?;

// let wasm_bytes = std::fs::read("./path/to/module.wasm")?;

let engine = JIT::new(&Cranelift::default()).engine();
let store = Store::new(&engine);
let module = Module::new(&store, wasm_bytes)?;

let import_object = imports! {};
let instance = Instance::new(&module, &import_object)?;

// A function named host_function in a namespace with an empty name;
// A global named host_global in the env namespace.
let import_object = imports! {
    "" => {
        "host_function" => host_function,
    },
    "env" => {
        "host_global" => host_global,
    },
};

let instance = Instance::new(&module, &import_object)?;
instance, err := wasmer.NewInstance(module, importObject)
instance = Instance(module, import_object)
wasm_instance_t* instance = wasm_instance_new(store, module, &import_object, NULL);


let function = instance.exports.get::<Function>("guest_function")?;
let global = instance.exports.get::<Global>("guest_global")?;
let memory = instance.exports.get::<Memory>("guest_memory")?;
let table = instance.exports.get::<Table>("guest_table")?;

