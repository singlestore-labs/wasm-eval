use wasm3::Environment;
use wasm3::Module;

const MILLIS: u64 = 500_000;

fn main() {
    let env = Environment::new().expect("Unable to create environment");
    let rt = env
        .create_runtime(1024 * 60)
        .expect("Unable to create runtime");
    let module = Module::parse(
        &env,
        &include_bytes!("wasm/wasm_millis_to_seconds/wasm_millis_to_seconds.wasm")[..],
    )
    .expect("Unable to parse module");

    let mut module = rt.load_module(module).expect("Unable to load module");
    module
        .link_closure("time", "millis", |_, ()| MILLIS)
        .expect("Unable to link closure");
    let func = module
        .find_function::<(), u64>("seconds")
        .expect("Unable to find function");
    println!("{}ms in seconds is {:?}s.", MILLIS, func.call());
    assert_eq!(func.call(), Ok(MILLIS / 1000));
}