use crate::bytecode::Bytecode2;
use crate::virtual_machine::bytecode;
use crate::virtual_machine::bytecode::usize_to_byte_array;
use crate::virtual_machine::chunk::{Chunk, Chunk2};
use crate::virtual_machine::table::{Table, TableKey};
use crate::virtual_machine::util::PTR_WIDTH;
use crate::virtual_machine::value::{HeapValue, Value};

pub struct Vm {
    chunks: Vec<Chunk>,
    tables: Vec<Table>,
    heap_variables: Vec<Value>,
    pub constants: Vec<Value>,
    register: [Value; 255],
}
impl Vm {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            tables: vec![],
            heap_variables: vec![],
            constants: vec![],
            register: [Value::EmptyTable; 255]
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
            //println!("instruction: {}", self.chunk().instructions.get(self.chunk().ptr).unwrap());
            self.chunk_mut().ptr += 1;
            match *self.chunk().instructions.get(self.chunk().ptr-1).unwrap() {
                bytecode::RETURN => {
                    let prev_stack = self.chunks.pop().unwrap().eval_stack;
                    self.chunk_mut().eval_stack.extend(prev_stack);
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
                bytecode::ALLOC_TABLE => {
                    self.tables.push(Table::default());
                    let table_ptr = Value::Int((self.tables.len()-1) as i64);
                    self.push_eval(table_ptr);
                }
                bytecode::TABLE_SET_INDEX => {
                    let table_ptr = val_to_usize(self.pop_eval()).unwrap();
                    let table_index = val_to_usize(self.pop_eval()).unwrap();
                    let value_to_insert = self.pop_eval();
                    let mut table = self.tables.get_mut(table_ptr).unwrap();
                    table.indexed_set(table_index, value_to_insert);
                }
                bytecode::TABLE_GET_INDEX => {
                    let table_ptr = val_to_usize(self.pop_eval()).unwrap();
                    let table_index = val_to_usize(self.pop_eval()).unwrap();
                    let mut table = self.tables.get_mut(table_ptr).unwrap();
                    let val = *table.indexed_get(table_index).unwrap();
                    self.push_eval(val);
                }
                bytecode::PEEK_LOCAL => {
                    let index = self.chunk_mut().index_from_stack();
                    let val = self.peek(index).unwrap();
                    self.push_eval(val);
                }
                bytecode::LOAD_INSTRUCTIONS => {
                    let table_ptr = val_to_usize(self.pop_eval()).unwrap();
                    let table = self.tables.get(table_ptr).unwrap();
                    let mut vals = vec![];
                    for pair in table.inner.iter() {
                        let val = *match pair {
                            TableKey::Str(_, val) => val,
                            TableKey::NoStr(val) => val,
                        };
                        vals.push(val);
                    }
                    let instructions = vals_to_u8_bytecode(vals);
                    //TODO add the ability to have certain things pushed to eval stack, or just use the eval stack of this whole thing, idk which.
                    let new_chunk = Chunk {
                        ptr: 0,
                        instructions,
                        locals: vec![],
                        eval_stack: vec![]
                    };
                    self.chunks.push(new_chunk);
                }
                bytecode::LOAD_CONST_NUM => {
                    let const_num = self.chunk_mut().index_from_stack();
                    self.push_eval(Value::Int(const_num as i64));
                }
                bytecode::REGISTER_SET => {
                    let register_index = self.chunk_mut().index_from_stack();
                    self.register[register_index] = self.pop_eval();
                }
                bytecode::REGISTER_GET => {
                    let register_index = self.chunk_mut().index_from_stack();
                    let value = *self.register.get(register_index).unwrap();
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
    pub fn peek(&self, distance: usize) -> Option<Value> {
        let pos = self.chunk().locals.len() - 1 - distance;
        match self.chunk().locals.get(pos) {
            None => None,
            Some(val) => Some(*val),
        }
    }
}
pub fn val_to_usize(value: Value) -> Option<usize> {
    match value {
        Value::Int(integer) => Some(integer as usize),
        _ => None,
    }
}
pub fn val_to_u8(value: Value) -> Option<Vec<u8>> {
    match value {
        Value::Int(integer) => {
            return if integer < 255 {
                Some(vec![integer as u8])
            } else {
                Some(usize_to_byte_array(integer as usize).to_vec())
            }
        },
        _ => None,
    }
}
pub fn vals_to_u8_bytecode(values: Vec<Value>) -> Vec<u8> {
    let mut bytes = vec![];
    let mut index = 0;
    while index < values.len() {
        match values.get(index) {
            None => unreachable!(),
            Some(value) => {
                let number = val_to_usize(*value).unwrap();
                bytes.push(number as u8);
                match number as u8 {
                    bytecode::LOAD_CONSTANT |
                    bytecode::LOAD_CONST_NUM |
                    bytecode::PEEK_LOCAL |
                    bytecode::REGISTER_GET |
                    bytecode::REGISTER_SET
                    => {
                        index += 1;
                        let new_number = val_to_usize(*values.get(index).unwrap()).unwrap();
                        bytes.extend(usize_to_byte_array(new_number));
                    }
                    _ => (),
                };
            }
        }
        index += 1;
    }
    bytes
}

pub struct Vm2 {
    pub heap: Vec<HeapValue>,
    pub chunks: Vec<Chunk2>,
}
impl Vm2 {
    pub fn new() -> Self {
        Self {
            heap: vec![],
            chunks: vec![]
        }
    }
    pub fn load(&mut self, chunk: Chunk2) {
        self.chunks.push(chunk);
    }
    pub fn unload(&mut self) {
        self.chunks.pop().unwrap();
    }
    pub fn chunk(&self) -> &Chunk2 {
        self.chunks.last().unwrap()
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk2 {
        self.chunks.last_mut().unwrap()
    }
    pub fn push(&mut self, value: Value) {
        self.chunk_mut().stack.push(value);
    }
    pub fn pop(&mut self) -> Value {
        self.chunk_mut().stack.pop().unwrap()
    }
    pub fn peek(&self, index: usize) -> Value {
        let len = self.chunk().stack.len();
        *self.chunk().stack.get(len - index).unwrap()
    }
    pub fn get_constant(&self, index: usize) -> Value {
        *self.chunk().constants.get(index).unwrap()
    }
    pub fn get_table(&mut self, index: usize) -> &mut Table {
        match self.heap.get_mut(index).unwrap() {
            HeapValue::Value(_) => panic!(),
            HeapValue::Table(table) => table,
        }
    }
    pub fn run(&mut self) {
        loop {
            if self.chunk().ptr == self.chunk().instructions.len() {
                break;
            }
            self.chunk_mut().ptr += 1;
            match *self.chunk().instructions.get(&self.chunk().ptr - 1).unwrap() {
                Bytecode2::AllocHeap => {
                    self.heap.push(HeapValue::Table(Table::default()));
                    self.push(Value::Int((self.heap.len()-1) as i64))
                },
                Bytecode2::LoadNumber(number) => {
                    self.push(Value::Int(number as i64))
                }
                Bytecode2::LoadConstant(const_index) => {
                    let value = self.get_constant(const_index);
                    self.push(value);
                }
                Bytecode2::LoadHeapValue(_) => todo!(),
                Bytecode2::LoadIndexHeapValue => todo!(),
                Bytecode2::Peek(_) => todo!(),
                Bytecode2::Pop => {
                    self.pop();
                }
                Bytecode2::HeapTableSetIndex => {
                    let value = self.peek(0);
                    let table_index = val_to_usize(self.peek(1)).unwrap();
                    let heap_index = val_to_usize(self.peek(2)).unwrap();
                    let table = self.get_table(heap_index);
                    table.indexed_set(table_index, value);
                }
                Bytecode2::HeapTableGetIndex => todo!(),
                Bytecode2::HeapTablePush => {
                    let value = self.peek(0);
                    let heap_index = val_to_usize(self.peek(2)).unwrap();
                    let table = self.get_table(heap_index);
                    table.inner.push(TableKey::NoStr(value));
                }
                Bytecode2::HeapTablePushWithKey => {
                    let value = self.peek(0);
                    let key = val_to_usize(self.peek(1)).unwrap();

                }
                Bytecode2::RegisterSet(_) => todo!(),
                Bytecode2::RegisterGet(_) => todo!(),
                Bytecode2::Jump(_) => todo!(),
                Bytecode2::JumpIf(_) => todo!(),
            }
        }
    }
}