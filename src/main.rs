use crate::bytecode::{ALLOC_TABLE, byte_array_to_usize, Bytecode};
use crate::Bytecode::{AllocTable, LoadConstNum, LoadInstructions, PeekLocal, PopLocal, PushLocal, RegisterSet, TableGetIndex, TableSetIndex};
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
    // vm.constants = vec![Value::Int(69420), Value::Int(0)];
    // vm.load(first_test_chunk());
    let (chunk, constants) = test_registers();
    vm.constants = constants;
    vm.load(chunk);
    vm.run();
}
fn first_test_chunk() -> Chunk {
    Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(vec![LoadConstant(0), Print, AllocTable, PushLocal, LoadConstant(0), LoadConstant(1), PeekLocal(0), TableSetIndex, LoadConstant(1), PopLocal, TableGetIndex, Print].as_slice()),
        locals: vec![],
        eval_stack: vec![],
    }
}
fn second_chunk() -> (Chunk, Vec<Value>) {
    let instructions = vec![
        AllocTable,
        PushLocal,
        LoadConstNum(bytecode::LOAD_CONSTANT as usize),
        LoadConstNum(0),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(0x0),
        LoadConstNum(1),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::PRINT as usize),
        LoadConstNum(2),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::RETURN as usize),
        LoadConstNum(3),
        PeekLocal(0),
        TableSetIndex,
        PeekLocal(0),
        LoadInstructions,
    ];
    let constants = vec![
        Value::Float(69.420),
    ];
    (Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(instructions.as_slice()),
        locals: vec![],
        eval_stack: vec![]
    }, constants)
}
pub fn test_registers() -> (Chunk, Vec<Value>) {
    let instructions = vec![
        LoadConstant(0x1),
        RegisterSet(0x10),
        AllocTable,
        PushLocal,
        LoadConstNum(bytecode::REGISTER_GET as usize),
        LoadConstNum(0),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(0x10),
        LoadConstNum(1),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::PRINT as usize),
        LoadConstNum(2),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::RETURN as usize),
        LoadConstNum(3),
        PeekLocal(0),
        TableSetIndex,
        PeekLocal(0),
        LoadInstructions
    ];
    let constants = vec![
        Value::Float(0.01),
        Value::Float(69.420),
    ];
    (Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(instructions.as_slice()),
        locals: vec![],
        eval_stack: vec![]
    }, constants)
}