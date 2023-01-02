mod misc;
mod bytecode;

use std::mem::size_of;
use std::ops::Add;
use indexmap::IndexMap;
use misc::Value;
use crate::second_attempt::vm2::bytecode::Bytecode::{AllocTable, Constant, InsertIndexTable, PeekLocal, PopLocal, Print, PushLocal};
use crate::second_attempt::vm2::bytecode::{CONSTANT, convert_back, convert_bytecode_array, convert_thing, PUSH_LOCAL};
use crate::second_attempt::vm2::misc::{Table, TableKey};
use crate::second_attempt::vm2::misc::Value::Number;

pub fn test_vm() {
    let instructions = vec![
        Constant(0),
        Print,
        AllocTable,
        PushLocal,
        Constant(1),
        Constant(0),
        PeekLocal(0),
        InsertIndexTable,
    ];
    let instructions = convert_bytecode_array(instructions);
    let constants = vec![
        Number(1),
        Number(21),
    ];
    let mut vm = Vm::default();
    vm.load(Chunk {
        ip: 0,
        instructions,
        constants
    });
    vm.run();
    //let val = 2;
    //let val = convert_thing(val);
    //println!("{:#?}", convert_back(val.as_slice()));
}
struct Vm {
    locals: Stack<256>,
    eval: Stack<256>,
    tables: Vec<Table>,
    chunks: Vec<Chunk>,
}
struct Chunk {
    ip: usize,
    instructions: Vec<u8>,
    constants: Vec<Value>,
}
impl Chunk {
    pub fn set_ip(&mut self, ip: usize) {
        self.ip = ip
    }
    pub fn get_ip(&self) -> usize {
        self.ip
    }
    pub fn get_instructions_mut(&mut self) -> &mut Vec<u8> {
        &mut self.instructions
    }
    pub fn get_constants_mut(&mut self) -> &mut Vec<Value> {
        &mut self.constants
    }
    pub fn get_instructions(&self) -> &Vec<u8> {
        &self.instructions
    }
    pub fn get_constants(&self) -> &Vec<Value> {
        &self.constants
    }
}
impl Vm {
    pub fn load(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn run(&mut self) {
        while self.get_ip() < self.get_instructions().len() {
            self.set_ip(self.get_ip()+1);
            //println!("Instructions: {}", self.get_instruction());
            match self.get_instruction() {
                bytecode::CONSTANT => {
                    let constant = self.get_constant();
                    self.move_index();
                    self.eval.push(constant);
                }
                bytecode::PRINT => {
                    let val = self.eval.pop().unwrap();
                    println!("{}", val);
                }
                bytecode::PUSH_LOCAL => {
                    let value = self.eval.pop().unwrap();
                    self.locals.push(value).unwrap();
                }
                bytecode::POP_LOCAL => {
                    let val = self.locals.pop().unwrap();
                    self.eval.push(val).unwrap();
                }
                bytecode::PEEK_LOCAL => {
                    let index = self.get_index();
                    self.move_index();
                    let val = self.locals.peek(index).unwrap();
                    self.eval.push(val);
                }
                bytecode::ALLOC_TABLE => {
                    self.tables.push(Table::Map(IndexMap::default()));
                    self.eval.push(Value::Number((self.tables.len()-1) as i64)).unwrap();
                }
                bytecode::INSERT_INDEX_TABLE => {
                    let index =
                    match self.eval.pop().unwrap() {
                        Number(number) => number as usize,
                        _ => panic!("not a number"),
                    };
                    let value_index =
                    match self.eval.pop().unwrap() {
                        Number(number) => number as usize,
                        _ => panic!("not a number"),
                    };
                    let value = self.eval.pop().unwrap();
                    let table = match self.tables.get_mut(index).unwrap() {
                        Table::Map(map) => map,
                        _ => panic!("not a table"),
                    };
                    table.insert(TableKey::NoIdentifier(value_index), value);
                }
                bytecode::GET_INDEX_TABLE => {
                    let index =
                    match self.eval.pop().unwrap() {
                      Number(number) => number as usize,
                        _ => panic!("not a number")
                    };
                    let value_index =
                    match self.eval.pop().unwrap() {
                        Number(number) => number as usize,
                        _ => panic!("not a number"),
                    };
                    let table = match self.tables.get_mut(index).unwrap() {
                        Table::Map(map) => map,
                        _ => panic!("not a table"),
                    };
                    let value = table.get(&TableKey::NoIdentifier(value_index)).unwrap();
                    self.eval.push(*value);
                }
                bytecode::INJECT => {
                    let table_index = match self.eval.pop().unwrap() {
                        Number(number) => number as usize,
                        _ => panic!("not a number"),
                    };
                    let table = self.tables.get(table_index).unwrap();
                    match table {
                        Table::Map(map) => {
                            let map: &IndexMap<TableKey, Value> = map;
                            let constants_map = map.get(&TableKey::Identifier(String::from("constants"), None)).unwrap();
                            let instructions_map = map.get(&TableKey::Identifier(String::from("instructions"), None)).unwrap();
                            let constants_map:&misc::Table = match constants_map {
                                Value::Table(index) => {
                                    self.tables.get(*index).unwrap()
                                }
                                _ => panic!()
                            };
                            let instructions_map: &misc::Table = match instructions_map {
                                Value::Table(index) => {
                                    self.tables.get(*index).unwrap()
                                }
                                _ => panic!()
                            };
                            let constants_map: &IndexMap<TableKey, Value> = match constants_map {
                                Table::Map(map) => map,
                                _ => panic!()
                            };
                            let instructions_map: &IndexMap<TableKey, Value> = match instructions_map {
                                Table::Map(map) => map,
                                _ => panic!()
                            };
                            let new_constants = constants_map.values().map(|value| {
                                *value
                            }).collect();
                            let new_instructions = instructions_map.values().map(|value| {
                                match value {
                                    Number(number) => *number as u8,
                                    _ => panic!(),
                                }
                            }).collect();
                            let new_chunk = Chunk {
                                ip: 0,
                                instructions: new_instructions,
                                constants: new_constants
                            };
                            self.chunks.push(new_chunk);
                        }
                        _ => panic!(),
                    }
                }
                bytecode::RETURN => {
                    self.chunks.pop().unwrap();
                }
                _ => {}
            }
        }
    }
    pub fn get_ip(&self) -> usize {
        self.get_chunk().get_ip()
    }
    pub fn set_ip(&mut self, ip: usize) {
        self.get_chunk_mut().set_ip(ip);
    }
    pub fn get_chunk(&self) -> &Chunk {
        self.chunks.last().unwrap()
    }
    pub fn get_chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }
    pub fn get_instructions(&self) -> &Vec<u8> {
        self.get_chunk().get_instructions()
    }
    pub fn get_constants(&self) -> &Vec<Value> {
        self.get_chunk().get_constants()
    }
    pub fn get_instructions_mut(&mut self) -> &mut Vec<u8> {
        self.get_chunk_mut().get_instructions_mut()
    }
    pub fn get_constants_mut(&mut self) -> &mut Vec<Value> {
        self.get_chunk_mut().get_constants_mut()
    }
    pub fn get_index(&self) -> usize {
        let index = convert_back(&self.get_instructions().as_slice()[self.get_ip()..self.get_ip()+size_of::<usize>()]);
        index
    }
    pub fn move_index(&mut self) {
        self.set_ip(self.get_ip() + size_of::<usize>())
    }
    pub fn get_table_mut(&mut self) -> &mut Table {
        let index = self.get_index();
        self.tables.get_mut(index).unwrap()
    }
    pub fn get_constant(&mut self) -> Value {
        let constant = *self.get_constants().get(self.get_index()).unwrap();
        self.move_index();
        constant
    }
    pub fn get_instruction(&self) -> u8 {
        *self.get_instructions().get(self.get_ip()-1).unwrap()
    }
}
impl Default for Vm {
    fn default() -> Self {
        Self {
            locals: Stack::default(),
            eval: Stack::default(),
            tables: vec![],
            chunks: vec![]
        }
    }
}

pub struct Stack<const N: usize> {
    stack: [Value; N],
    len: usize,
}
impl<const N: usize> Stack<N> {
    pub fn pop(&mut self) -> Option<Value> {
        if self.len == 0 {
            return None;
        }
        let val = self.stack[self.len-1];
        self.len -= 1;
        Some(val)
    }
    pub fn push(&mut self, value: Value) -> Option<()> {
        if self.len == N {
            return None;
        }
        self.stack[self.len] = value;
        self.len += 1;
        Some(())
    }
    pub fn peek(&mut self, index: usize) -> Option<Value> {
        Some(self.stack[self.compute_distance(index)?])
    }
    pub fn set(&mut self, index: usize, value: Value) -> Option<()> {
        self.stack[self.compute_distance(index)?] = value;
        Some(())
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    fn compute_distance(&self, index: usize) -> Option<usize> {
        if self.len - index < 0 {
            return None;
        }
        Some(self.len-index)
    }
}
impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        Self {
            stack: [Default::default(); N],
            len: 0
        }
    }
}