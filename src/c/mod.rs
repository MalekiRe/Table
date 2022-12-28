use crate::c::c_compiler::{compile, generate_c_file};
// use crate::c::transpile::comp_file;
use crate::ParserFile;

pub mod c_compiler;
mod transpile;
pub mod translation_wrapper;
mod transpile_2;

pub fn do_full_compilation(parser_file: ParserFile) {
    let mut c_file = generate_c_file();
    c_file.push_str(transpile_2::transpile(parser_file).as_str());
    //c_file.push_str(comp_file(parser_file).get_file().as_str());
    compile(c_file);
}