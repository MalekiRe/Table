use wasmtime::*;
pub fn wasmtime_runner(file: Vec<u8>) {
    let engine = Engine::default();
    let module = wasmtime::Module::from_binary(&engine, file.as_slice()).unwrap();
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add").unwrap();
    println!("result: {}", add.call(&mut store, (1, 2)).unwrap());
}