use util::PTR_WIDTH;
use crate::util;

pub const RETURN: u8 = 0x01;
pub const LOAD_CONSTANT: u8 = 0x02;
pub const PUSH_LOCAL: u8 = 0x03;
pub const POP_LOCAL: u8 = 0x04;
pub const PeekLocal: u8 = 0x05;
pub const PRINT: u8 = 0x06;
pub const LOAD_INSTRUCTIONS: u8 = 0x07;

pub enum Bytecode {
    Return,
    LoadConstant(usize),
    PushLocal,
    PopLocal,
    PeekLocal(usize),
    Print,
    LoadInstructions,
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
                    vec_bytes.push(PeekLocal);
                    vec_bytes.extend(usize_to_byte_array(*value).iter().map(|byte| *byte));
                }
                Bytecode::Print => vec_bytes.push(PRINT),
                Bytecode::LoadInstructions => vec_bytes.push(LOAD_INSTRUCTIONS),
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