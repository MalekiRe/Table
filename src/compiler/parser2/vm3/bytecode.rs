use crate::compiler::parser2::vm3::pointer::{ConstantPointer, LocalDistance, StackDistance};

#[derive(Debug, Clone)]
pub enum Bytecode {
    /// peeks `value` from `locals` and `stack.push(value)`
    PeekLocal(LocalDistance),
    /// pops `value` from `stack` and `locals.push(value)`
    PushLocal,
    /// `locals.pop()`
    PopLocal,
    DupStack(StackDistance),
    /// `stack.push(constants[constant_pointer])`
    LoadConstant(ConstantPointer),
    /// pops `heap_pointer` and `stack.push(heap[heap_pointer].into())` doesn't work for tables, strings, or chunks
    LoadHeap,
    // /// pops `stack_value` and `heap.push(stack_value.into())` and `stack.push(StackValue::HeapPointer(heap.len))`
    // UpValueStack,
    /// `local_value = locals[local_distance]` and `heap.push(local_value.into())` and `locals[local_value] = HeapPointer(heap.len() - 1)`
    UpValueLocal(LocalDistance),
    /// pops `stack_value` and `locals[local_distance] = stack_value`
    SetLocal(LocalDistance),
    /// pops `chunk_pointer` off the heap and runs it.
    LoadChunk,
    /// returns from chunk
    Return,
}