use crate::bytecode;
use crate::chunk::Chunk;
use crate::table::Table;
use crate::value::Value;

pub struct Vm {
    chunks: Vec<Chunk>,
    tables: Vec<Table>,
    heap_variables: Vec<Value>,
    pub constants: Vec<Value>,
}
impl Vm {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            tables: vec![],
            heap_variables: vec![],
            constants: vec![],
        }
    }
    pub fn load(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn unload(&mut self) {
        self.chunks.pop().unwrap();
    }
    pub fn run(&mut self) {
        while self.chunk_mut().ptr < self.chunk_mut().instructions.len() {
            self.chunk_mut().ptr += 1;
            match *self.chunk().instructions.get(self.chunk().ptr-1).unwrap() {
                bytecode::RETURN => {
                    unimplemented!()
                }
                bytecode::LOAD_CONSTANT => {
                    let index = self.chunk_mut().index_from_stack();
                    let constant = *self.constants.get(index).unwrap();
                    self.push_eval(constant);
                }
                bytecode::PRINT => {
                    println!("{}", self.pop_eval());
                }
                bytecode::PUSH_LOCAL => {
                    let value = self.pop_eval();
                    self.chunk_mut().locals.push(value);
                }
                bytecode::POP_LOCAL => {
                    let value = self.chunk_mut().locals.pop().unwrap();
                    self.push_eval(value);
                }
                _ => {}
            }
        }
    }
    pub fn chunk(&self) -> &Chunk {
        self.chunks.last().unwrap()
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }
    pub fn push_eval(&mut self, value: Value) {
        self.chunk_mut().eval_stack.push(value);
    }
    pub fn pop_eval(&mut self) -> Value {
        self.chunk_mut().eval_stack.pop().unwrap()
    }
}