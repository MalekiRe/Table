use crate::bytecode::byte_array_to_usize;
use crate::util::PTR_WIDTH;
use crate::value::Value;

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