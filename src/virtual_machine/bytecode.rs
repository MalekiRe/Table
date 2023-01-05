use util::PTR_WIDTH;
use crate::virtual_machine::util;

pub const RETURN: u8 = 0x01;
pub const LOAD_CONSTANT: u8 = 0x02;
pub const PUSH_LOCAL: u8 = 0x03;
pub const POP_LOCAL: u8 = 0x04;
pub const PEEK_LOCAL: u8 = 0x05;
pub const PRINT: u8 = 0x06;
pub const LOAD_INSTRUCTIONS: u8 = 0x07;
pub const ALLOC_TABLE: u8 = 0x08;
pub const TABLE_GET_INDEX: u8 = 0x09;
pub const TABLE_SET_INDEX: u8 = 0x0A;
pub const LOAD_CONST_NUM: u8 = 0x0B;
pub const REGISTER_SET: u8 = 0x0C;
pub const REGISTER_GET: u8 = 0x0D;

pub enum Bytecode {
    Return,
    LoadConstant(usize),
    LoadConstNum(usize),
    PushLocal,
    PopLocal,
    PeekLocal(usize),
    Print,
    LoadInstructions,
    AllocTable,
    TableGetIndex,
    TableSetIndex,
    RegisterSet(usize),
    RegisterGet(usize),
}

#[derive(Debug, PartialEq)]
pub enum Bytecode2 {
    /// adds to the heap vec, and pushes a usize for it's index onto the stack.
    AllocHeap,
    /// pushes this number onto the stack as a Value::Integer,
    LoadNumber(usize),
    /// grabs a constant at `index[usize]` and pushes onto the stack.
    LoadConstant(usize),
    /// grabs a Value at `heap[usize]` and pushes onto the stack.
    LoadHeapValue(usize),
    /// peeks the index of the heap and pushes the value at `heap[index]` onto the stack.
    LoadIndexHeapValue,
    /// peeks at value at `usize` length away from the top of the stack and pushes it onto the top of the stack
    Peek(usize),
    /// pops most recent value off of stack.
    Pop,
    /// peeks `value`, `table_index` and `heap_index` and `heap[heap_index][table_index] = value`
    HeapTableSetIndex,
    /// peeks and `table_index` and `heap_index` and pushes `heap[heap_index][table_index]`
    HeapTableGetIndex,
    /// peeks `value` and `heap_index` and `heap[heap_index].push(value)`
    HeapTablePush,
    /// pops `value` and `register[usize] = value`
    RegisterSet(usize),
    /// pushes `register[usize]`
    RegisterGet(usize),
    /// pops off jump position off stack
    Jump(usize),
    /// pops `jump_condition` and jumps if `jump_condition` true to `usize`
    JumpIf(usize),
}

impl Bytecode {
    pub fn convert_to_bytes(bytecode: &[Bytecode]) -> Vec<u8> {
        //TODO do this on the stack instead of the heap.
        let mut vec_bytes = Vec::<u8>::new();
        for code in bytecode {
            match code {
                Bytecode::Return => vec_bytes.push(RETURN),
                Bytecode::LoadConstant(value) => {
                    vec_bytes.push(LOAD_CONSTANT);
                    vec_bytes.extend(usize_to_byte_array(*value).iter().map(|byte| *byte));
                }
                Bytecode::PushLocal => vec_bytes.push(PUSH_LOCAL),
                Bytecode::PopLocal => vec_bytes.push(POP_LOCAL),
                Bytecode::PeekLocal(value) => {
                    vec_bytes.push(PEEK_LOCAL);
                    vec_bytes.extend(usize_to_byte_array(*value).iter().map(|byte| *byte));
                }
                Bytecode::Print => vec_bytes.push(PRINT),
                Bytecode::LoadInstructions => vec_bytes.push(LOAD_INSTRUCTIONS),
                Bytecode::AllocTable => vec_bytes.push(ALLOC_TABLE),
                Bytecode::TableGetIndex => vec_bytes.push(TABLE_GET_INDEX),
                Bytecode::TableSetIndex => vec_bytes.push(TABLE_SET_INDEX),
                Bytecode::LoadConstNum(value) => {
                    vec_bytes.push(LOAD_CONST_NUM);
                    vec_bytes.extend(usize_to_byte_array(*value))
                }
                Bytecode::RegisterSet(value) => {
                    vec_bytes.push(REGISTER_SET);
                    vec_bytes.extend(usize_to_byte_array(*value))
                }
                Bytecode::RegisterGet(value) => {
                    vec_bytes.push(REGISTER_GET);
                    vec_bytes.extend(usize_to_byte_array(*value))
                }
            }
        }
        vec_bytes
    }
}
pub fn usize_to_byte_array(usize: usize) -> [u8; PTR_WIDTH] {
    unsafe {
        std::mem::transmute::<usize, [u8; PTR_WIDTH]>(usize)
    }
}
pub fn byte_array_to_usize(byte_array: [u8; PTR_WIDTH]) -> usize {
    unsafe {
        std::mem::transmute::<[u8; PTR_WIDTH], usize>(byte_array)
    }
}