use crate::bytecode;
use crate::bytecode::usize_to_byte_array;
use crate::chunk::Chunk;
use crate::table::{Table, TableKey};
use crate::util::PTR_WIDTH;
use crate::value::Value;

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
            println!("instruction: {}", self.chunk().instructions.get(self.chunk().ptr).unwrap());
            self.chunk_mut().ptr += 1;
            match *self.chunk().instructions.get(self.chunk().ptr-1).unwrap() {
                bytecode::RETURN => {
                    self.chunks.pop();
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