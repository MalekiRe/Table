use crate::compiler::parser2::vm3::bytecode::{Bytecode};
use crate::compiler::parser2::vm3::chunk::Chunk;
use crate::compiler::parser2::vm3::pointer::{BooleanPointer, CharPointer, ChunkPointer, ConstantPointer, HeapPointer, LocalDistance, NumberPointer, StackDistance};
use crate::compiler::parser2::vm3::value::{HeapValue, StackValue};

pub struct Vm {
    chunks: Vec<Chunk>,
    heap: Vec<HeapValue>,
    pub local: Vec<StackValue>,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            chunks: vec![],
            heap: vec![],
            local: vec![]
        }
    }
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunks: vec![chunk],
            heap: vec![],
            local: vec![]
        }
    }
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn unload_chunk(&mut self) {
        let mut chunk = self.chunks.pop().unwrap();
        self.chunk_mut().stack.append(&mut chunk.stack);
    }
    pub fn chunk_ref(&self) -> &Chunk {
        self.chunks.last().unwrap()
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }

    pub fn find_constant(&self, constant_pointer: ConstantPointer) -> StackValue {
        *self.chunk_ref().constants.get::<usize>(constant_pointer.into()).unwrap()
    }
    pub fn push_stack(&mut self, stack_value: StackValue) {
        self.chunk_mut().stack.push(stack_value)
    }
    pub fn pop_stack(&mut self) -> StackValue {
        self.chunk_mut().stack.pop().unwrap()
    }
    pub fn peek_stack(&self, distance: StackDistance) -> StackValue {
        let distance: usize = distance.into();
        let len = self.chunk_ref().stack.len() - 1;
        *self.chunk_ref().stack.get(len - distance).unwrap()
    }

    pub fn peek_local(&self, distance: LocalDistance) -> StackValue {
        let distance: usize = distance.into();
        let len = self.local.len() - 1;
        *self.local.get(len - distance).unwrap()
    }
    pub fn push_local(&mut self, stack_value: StackValue) {
        self.local.push(stack_value);
    }
    pub fn pop_local(&mut self) -> StackValue {
        self.local.pop().unwrap()
    }
    pub fn set_local(&mut self, distance: LocalDistance, stack_value: StackValue) {
        let distance: usize = distance.into();
        let len = self.local.len() - 1;
        let index = len - distance;
        match self.local[index] {
            StackValue::HeapPointer(heap_pointer) => {
                let pointer: usize = heap_pointer.into();
                self.heap[pointer] = stack_value.try_into().unwrap();
            }
            _ => {self.local[index] = stack_value;}
        }
    }

    pub fn find_heap_value(&self, heap_pointer: HeapPointer) -> &HeapValue {
        let heap_pointer: usize = heap_pointer.into();
        &self.heap[heap_pointer]
    }
    pub fn find_heap_number(&self, number_pointer: NumberPointer) -> StackValue {
        let number_pointer: usize = number_pointer.into();
        match &self.heap[number_pointer] {
            HeapValue::Number(number) => StackValue::Number(*number),
            _ => panic!(),
        }
    }
    pub fn find_heap_bool(&self, bool_pointer: BooleanPointer) -> StackValue {
        todo!()
    }
    pub fn find_heap_char(&self, char_pointer: CharPointer) -> StackValue {
        todo!()
    }
    pub fn run(&mut self) {
        loop {
            if self.chunk_ref().instruction_ptr.val() == self.chunk_ref().bytecode.len() {
                break;
            }
            self.chunk_mut().instruction_ptr.increment();
            match self.chunk_ref().prev_bytecode() {
                Bytecode::PeekLocal(local_distance) => {
                    let mut stack_value = self.peek_local(local_distance);
                    match stack_value {
                        StackValue::HeapPointer(heap_pointer) => {
                            match heap_pointer {
                                HeapPointer::Number(number) => {
                                    stack_value = self.find_heap_number(number);
                                }
                                HeapPointer::Boolean(boolean) => {
                                    stack_value = self.find_heap_bool(boolean);
                                }
                                HeapPointer::Char(char) => {
                                    stack_value = self.find_heap_char(char);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    self.push_stack(stack_value);
                },
                Bytecode::PushLocal => {
                    let stack_value = self.pop_stack();
                    self.push_local(stack_value);
                },
                Bytecode::PopLocal => {
                    self.pop_local();
                },
                Bytecode::DupStack(_) => todo!(),
                Bytecode::LoadConstant(constant_pointer) => {
                    let stack_value = self.find_constant(constant_pointer);
                    self.push_stack(stack_value);
                },
                Bytecode::LoadHeap => {
                    let heap_pointer: HeapPointer = self.pop_stack().try_into().unwrap();
                    let stack_value: StackValue = self.find_heap_value(heap_pointer).try_into().unwrap();
                    self.push_stack(stack_value);
                },
                Bytecode::UpValueLocal(local_distance) => {
                    let stack_value = self.peek_local(local_distance.clone());
                    let heap_value = match stack_value {
                        StackValue::Nil => panic!("don't upvalue nils"),
                        StackValue::Number(number) => HeapValue::Number(number),
                        StackValue::Char(char) => HeapValue::Char(char),
                        StackValue::Boolean(bool) => HeapValue::Boolean(bool),
                        StackValue::HeapPointer(_) => panic!("impossible"),
                    };
                    let index = self.heap.len();
                    self.heap.push(heap_value);
                    let new_stack_value = StackValue::HeapPointer(
                      match stack_value {
                          StackValue::Number(_) => HeapPointer::Number(NumberPointer(index)),
                          StackValue::Char(_) => HeapPointer::Char(CharPointer(index)),
                          StackValue::Boolean(_) => HeapPointer::Boolean(BooleanPointer(index)),
                          _ => panic!(),
                      }
                    );
                    self.set_local(local_distance, new_stack_value);

                },
                Bytecode::SetLocal(local_distance) => {
                    let stack_value = self.pop_stack();
                    self.set_local(local_distance, stack_value);
                },
                Bytecode::LoadChunk => {
                    let chunk_pointer: ChunkPointer = self.pop_stack().try_into().unwrap();
                    let chunk = self.chunk_mut().chunks.get::<usize>(chunk_pointer.into()).unwrap().clone();
                    self.load_chunk(chunk);
                },
                Bytecode::Return => {
                    self.unload_chunk();
                },
            }
        }
    }
}

#[cfg(test)]
mod virtual_machine_test {
    use crate::compiler::parser2::vm3::bytecode::Bytecode;
    use crate::compiler::parser2::vm3::chunk::Chunk;
    use crate::compiler::parser2::vm3::pointer::{ConstantPointer, LocalDistance};
    use crate::compiler::parser2::vm3::value::{HeapValue, StackValue};
    use crate::compiler::parser2::vm3::vm::Vm;

    #[test]
    pub fn simple() {
        let bytecode = vec![
            Bytecode::LoadConstant(ConstantPointer(0)),
            Bytecode::PushLocal,
            Bytecode::LoadConstant(ConstantPointer(1)),
            Bytecode::PushLocal,
            Bytecode::PeekLocal(LocalDistance(1)),
        ];
        let constants = vec![
            StackValue::Boolean(false),
            StackValue::Number(1.0),
        ];
        let mut vm = Vm::new(Chunk::from(bytecode, constants));
        vm.run();
        assert_eq!(vm.chunk_ref().stack, vec![StackValue::Boolean(false)]);
    }
    #[test]
    pub fn upvalue() {
        let bytecode = vec![
            Bytecode::LoadConstant(ConstantPointer(0)),
            Bytecode::PushLocal,
            Bytecode::LoadConstant(ConstantPointer(1)),
            Bytecode::PushLocal,
            Bytecode::UpValueLocal(LocalDistance(1)),
            Bytecode::LoadConstant(ConstantPointer(1)),
            Bytecode::SetLocal(LocalDistance(1))
        ];
        let constants = vec![
          StackValue::Number(1.0),
            StackValue::Char('h'),
        ];
        let mut vm = Vm::new(Chunk::from(bytecode, constants));
        vm.run();
        match vm.heap.pop().unwrap() {
            HeapValue::Char(number) => {
                assert_eq!(number, 'h')
            }
            _ => panic!(),
        }
    }
}