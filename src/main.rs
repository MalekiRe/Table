use crate::bytecode::{byte_array_to_usize, Bytecode};
use crate::bytecode::Bytecode::{LoadConstant, Print};
use crate::chunk::Chunk;
use crate::value::Value;
use crate::vm::Vm;

pub mod bytecode;
pub mod util;
pub mod vm;
pub mod table;
pub mod value;
mod chunk;

fn main() {
    let mut vm = Vm::new();
    vm.constants = vec![Value::Int(69420)];
    vm.load(first_test_chunk());
    vm.run();
}
fn first_test_chunk() -> Chunk {
    Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(vec![LoadConstant(0), Print].as_slice()),
        locals: vec![],
        eval_stack: vec![],
    }
}
