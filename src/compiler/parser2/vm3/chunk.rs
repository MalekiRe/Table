
use crate::compiler::parser2::vm2::pointers::InstructionPointer;
use crate::compiler::parser2::vm3::bytecode::Bytecode;
use crate::compiler::parser2::vm3::value::StackValue;


#[derive(Debug, Clone)]
pub struct Chunk {
    pub instruction_ptr: InstructionPointer,
    pub bytecode: Vec<Bytecode>,
    pub constants: Vec<StackValue>,
    pub stack: Vec<StackValue>,
    pub chunks: Vec<Chunk>,
}
impl Default for Chunk {
    fn default() -> Self {
        Self::from(vec![], vec![])
    }
}
impl Chunk {
    pub fn from(bytecode: Vec<Bytecode>, constants: Vec<StackValue>) -> Self {
        Self {
            instruction_ptr: InstructionPointer(0),
            bytecode,
            constants,
            stack: vec![],
            chunks: vec![]
        }
    }
    pub fn prev_bytecode(&self) -> Bytecode {
        self.bytecode.get(self.instruction_ptr.val()-1).unwrap().clone()
    }
}