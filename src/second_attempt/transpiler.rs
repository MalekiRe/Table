use std::path::Path;
use crate::second_attempt::ir;
use crate::second_attempt::ir::File;

pub fn transpile(file: ir::File) -> String {
    let mut buffer = String::from(r#"
        #include "walloc.h"
        #include "shared_std.h"
        #include "table_std.h"

        Value _main();

        void _start() {
            print_value(_main());
        }

    "#);
    buffer.push_str(entry_point_file(file).as_str());
    buffer
}
pub fn entry_point_file(file: ir::File) -> String {
    /*
    Okay so the plan.
    We will wrap every `block` in `{` `}`
    We will save every FnDef and split them as a header and the body, and put the header at the
    top of the file.
    We will ignore lazy evaluation for now.
    We create a new variable declaration for each implicit/inline declaration.
    We track each implicit and explicit variable declaration.
    We append a ref_dec for each implicit and explicit variable unless it's returned.
    For inline variables we append `_inline` to it.
    For all variables we append `_{TYPE}` to it for the type it is, just for readability
    Just gonna heap allocate everything for now
     */
    String::from(r#"
        Value _main() {
            print("hi");
            return test();
        }
    "#)
}
pub fn compile_c_file<P: AsRef<std::path::Path>>(path: P, header_file_paths: Vec<&Path>) -> String {
    use std::os::unix::prelude::CommandExt;
    use std::fs;
    let path = path.as_ref();
    let file_bytes = fs::read(path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    fs::write(format!("target/{}", file_name), file_bytes).unwrap();

    let mut header_file_names = vec![];
    for header_file_path in header_file_paths {
        let header_file_bytes = fs::read(header_file_path).unwrap();
        let header_file_name = header_file_path.file_name().unwrap().to_str().unwrap();
        fs::write(format!("target/{}", header_file_name), header_file_bytes).unwrap();
        header_file_names.push(header_file_name);
    }

    let mut command = std::process::Command::new("clang");
        command
        .current_dir("target")
        .arg("--target=wasm32-unknown-wasi")
        .arg("--no-standard-libraries")
        .arg("-fno-builtin")
        .arg("-c")
        .arg(file_name);
    for header_file_name in header_file_names {
        command.arg("-include");
        command.arg(header_file_name);
    }
    command.spawn().unwrap().wait().unwrap();
    let mut file_name = file_name.to_string();
    file_name.pop();
    file_name.push('o');
    file_name
}
pub fn process_header_file<P: AsRef<std::path::Path>>(path: P) -> String {
    use std::os::unix::prelude::CommandExt;
    use std::fs;
    let path = path.as_ref();
    let file_bytes = fs::read(path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    fs::write(format!("target/{}", file_name), file_bytes).unwrap();
    file_name.to_string()
}
pub fn compile_c_files(c_files: Vec<&Path>, output: Vec<u8>, headers: Vec<&Path>) {
    let mut file_names = vec![];
    for file in c_files {
        file_names.push(compile_c_file(file, vec![]))
    }
    std::fs::write("target/main_file.c", output).unwrap();
    file_names.push(compile_c_file("target/main_file.c", headers));
    use std::os::unix::prelude::CommandExt;
    std::process::Command::new("clang")
        .current_dir("target")
        .arg("--target=wasm32-unknown-wasi")
        .arg("--no-standard-libraries")
        .arg("-fno-builtin")
        .args(file_names)
        .arg("-Wl,--export-all")
        .arg("-Wl,--no-entry")
        .arg("-o")
        .arg("output.wasm")
        .spawn().unwrap().wait().unwrap();
}