use std::fs::File;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::Command;

const BEGIN: &'static str = include_str!("basic_things.c");

pub fn generate_c_file() -> String {
    BEGIN.to_string()
}
pub fn compile(file: String) {
    let mut compiled_file = File::create("target/full_compilation.c").unwrap();
    compiled_file.write(file.as_bytes()).unwrap();
    let command = Command::new("clang")
        .current_dir("target")
        .arg("--target=wasm32-unknown-wasi")
        .arg("--no-standard-libraries")
        .arg("-Wl,--export-all")
        .arg("-Wl,--no-entry")
        .arg("-o")
        .arg("add.wasm")
        .arg("full_compilation.c")
        .spawn().unwrap().wait().unwrap();
}