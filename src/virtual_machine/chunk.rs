use crate::bytecode::Bytecode2;
use crate::virtual_machine::bytecode::byte_array_to_usize;
use crate::virtual_machine::util::PTR_WIDTH;
use crate::virtual_machine::value::Value;

pub struct Chunk {
    pub ptr: usize,
    pub instructions: Vec<u8>,
    pub locals: Vec<Value>,
    pub eval_stack: Vec<Value>,
}
impl Chunk {
    pub fn index_from_stack(&mut self) -> usize {
        let usize = byte_array_to_usize(self.instructions[self.ptr..self.ptr+PTR_WIDTH].try_into().unwrap());
        self.ptr += PTR_WIDTH;
        usize
    }
}
pub struct Chunk2 {
    pub ptr: usize,
    pub instructions: Vec<Bytecode2>,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
}
impl Chunk2 {
    pub fn new(instructions: Vec<Bytecode2>, constants: Vec<Value>) -> Self {
        Self {
            ptr: 0,
            instructions,
            stack: vec![],
            constants
        }
    }
}