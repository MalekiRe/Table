use std::fs;
use crate::main;
use crate::second_attempt::ir::File;
use crate::second_attempt::transpiler::{compile_c_file, compile_c_files};
use crate::wasm::wasmtime_runner;

pub mod ir;
pub mod lexer;
mod transpiler;
mod ir_to_string;
mod ir_to_string_2;
mod test_transpiler;
mod c_gen_helper;
mod ir3;
mod parser;

pub fn new_entrypoint(string: String) {
    //let file = transpiler::transpile(File::None);
    //compile_files(Some(file.into_bytes()));
    //wasmtime_runner(fs::read("target/output.wasm").unwrap());
    //test_transpiler::test_transpiler();
    test_transpiler::test_parser(string);
}
fn to_paths(str: Vec<&str>) -> Vec<&std::path::Path> {
    str.into_iter().map(|str| {
        std::path::Path::new(str)
    }).collect()
}
fn compile_files(main_file: Option<Vec<u8>>) {
    use std::path::Path;
    let mut c_file_names = vec![];
    c_file_names.push(compile_c_file("src/second_attempt/c_files/walloc.c",
                                     to_paths(vec!["src/second_attempt/c_files/walloc.h", "src/second_attempt/c_files/shared_std.h"])));
    c_file_names.push(compile_c_file("src/second_attempt/c_files/table_std.c",
                                     to_paths(vec!["src/second_attempt/c_files/table_std.h", "src/second_attempt/c_files/walloc.h", "src/second_attempt/c_files/shared_std.h"])));
    match main_file {
        None => {}
        Some(main_file) => {
            std::fs::write("target/main_file.c", main_file).unwrap();
            c_file_names.push(compile_c_file("target/main_file.c",
            to_paths(vec![
                "src/second_attempt/c_files/table_std.h",
                "src/second_attempt/c_files/shared_std.h",
                "src/second_attempt/c_files/walloc.h",
            ])))
        }
    }
    use std::os::unix::prelude::CommandExt;
    std::process::Command::new("clang")
        .current_dir("target")
        .arg("--target=wasm32-unknown-wasi")
        .arg("--no-standard-libraries")
        .args(c_file_names)
        .arg("-Wl,--export-all")
        .arg("-Wl,--no-entry")
        .arg("-o")
        .arg("output.wasm")
        .spawn().unwrap().wait().unwrap();
}