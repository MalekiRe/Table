use crate::compiler::parser2::vm2::bytecode::Bytecode;
use crate::compiler::parser2::vm2::chunk::Chunk;
use crate::compiler::parser2::vm2::pointers::{ConstantPointer, HeapPointer, LocalPointer, StackPointer};
use crate::compiler::parser2::vm2::value::{HeapValue, StackValue};

pub struct Vm {
    chunks: Vec<Chunk>,
    heap: Vec<HeapValue>,
    pub local: Vec<StackValue>,
}
impl Vm {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            heap: vec![],
            local: vec![]
        }
    }
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn unload_chunk(&mut self) {
        self.chunks.pop().unwrap();
    }
    pub fn chunk(&self) -> &Chunk {
        self.chunks.last().unwrap()
    }
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }
    pub fn find_constant(&self, constant_pointer: ConstantPointer) -> StackValue {
        *self.chunk().constants.get::<usize>(constant_pointer.into()).unwrap()
    }
    pub fn heap_val_ref(&self, heap_pointer: HeapPointer) -> &HeapValue {
        self.heap.get::<usize>(heap_pointer.into()).unwrap()
    }
    pub fn heap_val_ref_mut(&mut self, heap_pointer: HeapPointer) -> &mut HeapValue {
        self.heap.get_mut::<usize>(heap_pointer.into()).unwrap()
    }

    pub fn push_stack(&mut self, stack_value: StackValue) {
        self.chunk_mut().stack.push(stack_value)
    }
    pub fn pop_stack(&mut self) -> StackValue {
        self.chunk_mut().stack.pop().unwrap()
    }
    pub fn find_stack(&self, stack_pointer: StackPointer) -> StackValue {
        *self.chunk().stack.get::<usize>(stack_pointer.into()).unwrap()
    }
    pub fn peek_stack(&self, distance: usize) -> StackValue {
        let len = self.chunk().stack.len() - 1;
        *self.chunk().stack.get(len - distance).unwrap()
    }
    pub fn peek_head_stack(&self) -> StackValue {
        *self.chunk().stack.last().unwrap()
    }

    pub fn push_local(&mut self, stack_value: StackValue) {
        self.local.push(stack_value);
    }
    pub fn pop_local(&mut self) -> StackValue {
        self.local.pop().unwrap()
    }
    pub fn find_local(&self, stack_pointer: LocalPointer) -> StackValue {
        *self.local.get::<usize>(stack_pointer.into()).unwrap()
    }
    pub fn peek_local(&self, distance: usize) -> StackValue {
        let len = self.local.len() - 1;
        *self.chunk().stack.get(len - distance).unwrap()
    }
    pub fn run(&mut self) {
        loop {
            if self.chunk().instruction_ptr.val() == self.chunk().bytecode.len() {
                break;
            }
            self.chunk_mut().instruction_ptr.increment();
            match self.chunk().prev_bytecode() {
                Bytecode::DupAt(distance) => {
                    let stack_value = self.peek_stack(distance);
                    self.push_stack(stack_value);
                }
                Bytecode::Dup => {
                    let stack_value = self.peek_head_stack();
                    self.push_stack(stack_value);
                }
                Bytecode::Pop => {
                    self.pop_stack();
                }
                Bytecode::JumpIf(_) => {}
                Bytecode::LoadConstant(constant_pointer) => {
                    let stack_value = self.find_constant(constant_pointer);
                    self.push_stack(stack_value);
                }
                Bytecode::LoadHeap(heap_pointer) => {
                    let heap_value = self.heap_val_ref(heap_pointer);
                    let stack_value = heap_value.try_to_stack_value().unwrap();
                    self.push_stack(stack_value);
                }
                Bytecode::AllocTable => {}
                Bytecode::AllocString => {}
                Bytecode::AllocValue => {
                    let stack_value = self.peek_head_stack();
                    self.heap.push(stack_value.try_to_heap_value().unwrap());
                }
                Bytecode::PeekLocal(distance) => todo!(),
                Bytecode::FindLocal(local_pointer) => {
                    let stack_value = self.find_local(local_pointer);
                    self.push_stack(stack_value);
                }
                Bytecode::PushLocal => {
                    let stack_value = self.peek_head_stack();
                    self.push_local(stack_value);
                }
                Bytecode::PopLocal => {
                    self.pop_local();
                }
                Bytecode::SetLocal(local_pointer) => {
                    let stack_value = self.peek_head_stack();
                    let pointer: usize = local_pointer.into();
                    self.local[pointer] = stack_value;
                }
                Bytecode::AllocLocal(_) => {}
                Bytecode::Add => {}
                Bytecode::Eq => {}
                Bytecode::Invert => {}
                Bytecode::GetTableNum => {}
                Bytecode::GetTableStr => {}
                Bytecode::SetTableNum => {}
                Bytecode::SetTableStr => {}
                Bytecode::PushTableNum => {}
                Bytecode::PushTableStr => {}
                Bytecode::PushChar => {}
            }
        }
    }
}