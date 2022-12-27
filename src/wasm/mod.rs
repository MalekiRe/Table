use std::ffi::{c_char, CStr, CString};
use wasmtime::*;
pub fn wasmtime_runner(file: Vec<u8>) {
    let engine = Engine::default();
    let module = wasmtime::Module::from_binary(&engine, file.as_slice()).unwrap();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host", "print", |mut caller: Caller<'_, ()>, param: u32| {
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        unsafe {
            let ptr = memory.data_ptr(caller).add(param as usize);
            let c_str = CStr::from_ptr(ptr as *mut c_char);
            print!("{}", c_str.to_str().unwrap())
        }
    }).unwrap();
    linker.func_wrap("host", "print_num", |mut caller: Caller<'_, ()>, first: i32, second: i32| {
        print!("big number: {}.{}", first, second);
    }).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start").unwrap();
    start.call(&mut store, ()).unwrap();
}