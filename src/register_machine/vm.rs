use std::borrow::BorrowMut;
use std::collections::HashMap;
use simple_error::SimpleError;
use crate::bytecode::Bytecode2;
use crate::ir::IdentifierT;
use crate::register_machine::stack_value::StackValue;
use crate::register_machine::vm::Bytecode::Pop;

#[derive(Debug)]
pub struct Table {
    identifier_map: HashMap<IdentifierT, u32>,
    values: Vec<StackValue>,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            identifier_map: Default::default(),
            values: vec![]
        }
    }
}
impl Table {
    pub fn get_with_ident(&self, identifier: &str) -> StackValue {
        self.values[*self.identifier_map.get(identifier).unwrap() as usize]
    }
    pub fn get(&self, index: u32) -> StackValue {
        self.values[index as usize]
    }
    pub fn push_with_ident(&mut self, identifier: IdentifierT, value: StackValue) -> u32{
        let index = self.values.len() as u32;
        self.identifier_map.insert(identifier, index);
        self.values.push(value);
        index
    }
    pub fn push(&mut self, value: StackValue) -> u32 {
        let index = self.values.len() as u32;
        self.values.push(value);
        index
    }
    pub fn replace_with_ident(&mut self, identifier: &str, value: StackValue) -> u32 {
        let index = *self.identifier_map.get(identifier).unwrap();
        self.values[index as usize] = value;
        index
    }
    pub fn replace(&mut self, index: u32, value: StackValue) {
        self.values[index as usize] = value;
    }
    pub fn remove_with_ident(&mut self, identifier: &str) {
        let index = self.identifier_map.remove(identifier).unwrap();
        self.values[index as usize] = StackValue::Nil;
    }
    pub fn remove(&mut self, index: u32) {
        self.values[index as usize] = StackValue::Nil;
    }
}



#[derive(Debug)]
pub enum HeapValue {
    Number(f32),
    String(String),
    Table(Table),
    Boolean(bool),
    Nil,
}
#[derive(Debug)]
pub enum Bytecode {
    /// takes the value `distance = u32` and `stack.peek(distance)`
    DupAt(u32),
    /// dups the top of the stack.
    Dup,
    /// pops `value` off stack and does nothing with it.
    Pop,
    /// pops `condition`
    JumpIf(u32),
    /// pushes `constant_vars[u32]` onto stack
    LoadConstant(u32),
    /// pushes `Heap[u32]` onto stack, doesn't work if it's a `String` or `Table`
    LoadHeap(u32),
    /// pushes `Heap.push(Table::new())` and pushes `Heap.len()-1` onto stack
    AllocTable,
    /// peeks `value` an pushes it onto heap and pushes `heap_index` onto stack
    AllocValue,
    /// pushes `local_vars[value]` onto stack
    PeekLocal(u32),
    /// peeks `value` and `local_vars.push(value)`
    PushLocal,
    /// `local_vars.pop()` and pushes onto stack
    PopLocal,
    /// `local = local_vars[u32]` and then `heap.push(local)` and then pushes `heap.len()-1` onto stack
    AllocLocal(u32),
    /// pops `rhs` `lhs` and pushes `lhs + rhs` onto stack
    AddPop,
    /// pops `rhs` `lhs` and pushes `lhs == rhs` onto stack
    EqPop,
    /// pops `value` and pushes `!value` onto stack
    InvertPop,
    /// pops `table_index` `heap_index` and pushes `Heap[heap_index][table_index]`
    GetTableNum,
    /// peeks `str_index` `heap_index` and pushes `Heap[heap_index].get(Heap[str_index])`
    GetTableStr,
    /// peeks `value` `table_index` `heap_index` and `Heap[heap_index][table_index] = value`
    SetTableNum,
    /// peeks `value` `str_index` `heap_index` and `Heap[heap_index].get(Heap[str_index]) = value`
    SetTableStr,
    /// pops `value` peeks `heap_index` and `table = Heap[heap_index]` `table.push(value)` and pushes `table.len()-1` onto stack
    PushTableNum,
    /// pops `value` peeks `str_index` `heap_index` `table = Heap[heap_index]` `table.insert(str_index, value)` and pushes `table.len()-1` onto stack
    PushTableStr,
}
#[derive(Debug)]
pub struct Chunk {
    ptr: u32,
    bytecode: Vec<Bytecode>,
    constants: Vec<StackValue>,
    pub stack: Vec<StackValue>,
}
impl Chunk {
    pub fn from(bytecode: Vec<Bytecode>, constants: Vec<StackValue>) -> Self {
        Self {
            ptr: 0,
            bytecode,
            constants,
            stack: vec![]
        }
    }
}
pub struct Vm {
    chunks: Vec<Chunk>,
    heap: Vec<HeapValue>,
    local: Vec<StackValue>,
}
impl Vm {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            heap: vec![],
            local: vec![]
        }
    }
    pub fn load(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn unload(&mut self) {
        self.chunks.pop();
    }
    pub fn chunk(&self) -> &Chunk {
        self.chunks.last().unwrap()
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }
    pub fn get_constant(&self, index: u32) -> StackValue {
        *self.chunk().constants.get(index as usize).unwrap()
    }
    pub fn get_heap(&self, index: u32) -> &HeapValue {
        self.heap.get(index as usize).unwrap()
    }
    pub fn get_heap_mut(&mut self, index: u32) -> &mut HeapValue {
        self.heap.get_mut(index as usize).unwrap()
    }
    pub fn pop(&mut self) -> StackValue {
        self.chunk_mut().stack.pop().unwrap()
    }
    pub fn push(&mut self, value: StackValue) {
        self.chunk_mut().stack.push(value);
    }
    pub fn peek(&self, distance: u32) -> StackValue {
        let index = (self.chunk().stack.len() - distance as usize) - 1;
        *self.chunk().stack.get(index).unwrap()
    }
    pub fn pop_local(&mut self) -> StackValue {
        self.local.pop().unwrap()
    }
    pub fn push_local(&mut self, value: StackValue) {
        self.local.push(value)
    }
    pub fn peek_local(&mut self, index: u32) -> StackValue {
        *self.local.get(index as usize).unwrap()
    }
    pub fn run(&mut self) {
        loop {
            if self.chunk().ptr == self.chunk().bytecode.len() as u32 {
                break;
            }
            self.chunk_mut().ptr += 1;
            match *self.chunk().bytecode.get((self.chunk().ptr - 1) as usize).unwrap() {
                Bytecode::DupAt(distance) => {
                    self.push(self.peek(distance))
                }
                Bytecode::Dup => {
                    self.push(self.peek(0));
                }
                Bytecode::Pop => {
                    self.chunk_mut().stack.pop();
                }
                Bytecode::JumpIf(position) => {
                    let condition: bool = self.pop().try_into().unwrap();
                    if condition {
                        self.chunk_mut().ptr = position;
                    }
                },
                Bytecode::LoadConstant(index) => {
                    self.push(self.get_constant(index))
                },
                Bytecode::LoadHeap(index) => {
                    let value = match self.get_heap(index) {
                        HeapValue::Number(number) => StackValue::Number(*number),
                        HeapValue::Boolean(boolean) => StackValue::Boolean(*boolean),
                        HeapValue::Nil => StackValue::Nil,
                        HeapValue::String(_) | HeapValue::Table(_) => panic!(),
                    };
                    self.push(value);
                },
                Bytecode::AllocTable => {
                    self.heap.push(HeapValue::Table(Table::default()));
                    self.push(StackValue::Table((self.heap.len() - 1) as u32));
                },
                Bytecode::AllocValue => {
                    let value = self.peek(0);
                    self.heap.push(value.try_into().unwrap());
                    self.push((self.heap.len() - 1).try_into().unwrap());
                },
                Bytecode::PeekLocal(index) => {
                    let val = self.peek_local(index);
                    self.push(val);
                },
                Bytecode::PushLocal => {
                    let val = self.peek(0);
                    self.push_local(val);
                },
                Bytecode::PopLocal => {
                    let val = self.pop_local();
                    self.push(val);
                },
                Bytecode::AllocLocal(index) => {
                    let val = self.peek_local(index);
                    self.heap.push(val.try_into().unwrap());
                },
                Bytecode::AddPop => {
                    let rhs: f32 = self.pop().try_into().unwrap();
                    let lhs: f32 = self.pop().try_into().unwrap();
                    self.push((lhs + rhs).try_into().unwrap())
                },
                Bytecode::EqPop => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    let result = match rhs {
                        StackValue::Number(rhs) => {
                            match lhs {
                                StackValue::Number(lhs) => {
                                    rhs == lhs
                                }
                                _ => panic!(),
                            }
                        }
                        StackValue::Boolean(rhs) => {
                            match lhs {
                                StackValue::Boolean(lhs) => {
                                    rhs == lhs
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    };
                    self.push(result.try_into().unwrap());
                },
                Bytecode::InvertPop => {
                    let value: bool = self.pop().try_into().unwrap();
                    self.push((!value).try_into().unwrap());
                },
                Bytecode::GetTableNum => {
                    let table_index: u32 = self.pop().try_into().unwrap();
                    let heap_index: usize = self.pop().try_to_table_index().unwrap();
                    let table: &Table = self.heap.get(heap_index).unwrap().try_into().unwrap();
                    let value = table.get(table_index);
                    self.push(value);
                },
                Bytecode::GetTableStr => {
                    let str_index: usize = self.peek(0).try_into().unwrap();
                    let heap_index: usize = self.peek(1).try_into().unwrap();
                    let table: &Table = self.heap.get(heap_index).unwrap().try_into().unwrap();
                    let str_val: &String = self.heap.get(str_index).unwrap().try_into().unwrap();
                    let value = table.get_with_ident(str_val.as_str());
                    self.push(value);
                },
                Bytecode::SetTableNum => {
                    let value = self.peek(0);
                    let table_index: u32 = self.peek(1).try_into().unwrap();
                    let heap_index: usize = self.peek(2).try_into().unwrap();
                    let table: &mut Table = self.heap.get_mut(heap_index).unwrap().try_into().unwrap();
                    table.replace(table_index, value);
                },
                Bytecode::SetTableStr => {
                    let value = self.peek(0);
                    let str_index: usize = self.peek(1).try_into().unwrap();
                    let heap_index: usize = self.peek(2).try_into().unwrap();
                    let str_val: &String = self.heap.get(str_index).unwrap().try_into().unwrap();
                    let str_val = str_val.clone();
                    let table: &mut Table = self.heap.get_mut(heap_index).unwrap().try_into().unwrap();
                    table.replace_with_ident(str_val.as_str(), value);
                },
                Bytecode::PushTableNum => {
                    let value = self.pop();
                    let heap_index: usize = self.peek(0).try_to_table_index().unwrap();
                    let table: &mut Table = self.heap.get_mut(heap_index).unwrap().try_into().unwrap();
                    table.push(value);
                    let index = table.values.len()-1;
                    self.push(StackValue::Number(index as f32))
                },
                Bytecode::PushTableStr => {
                    let value = self.pop();
                    let str_index: usize = self.peek(0).try_into().unwrap();
                    let heap_index: usize = self.peek(1).try_into().unwrap();
                    let str: &String = self.heap.get(str_index).unwrap().try_into().unwrap();
                    let str = str.clone();
                    let table: &mut Table = self.heap.get_mut(heap_index).unwrap().try_into().unwrap();
                    table.push_with_ident(str, value);
                },
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::register_machine::stack_value::StackValue;
    use crate::register_machine::vm::{Bytecode, Chunk, Vm};

    #[test]
    fn first() {
        let chunk = Chunk::from(vec![
            Bytecode::LoadConstant(0),
            Bytecode::LoadConstant(1),
            Bytecode::AddPop,
        ], vec![
            StackValue::Number(0.1),
            StackValue::Number(0.2),
        ]);
        let mut vm = Vm::new();
        vm.load(chunk);
        vm.run();
        assert_eq!(*vm.chunk().stack.first().unwrap(), StackValue::Number(0.3));
    }
}