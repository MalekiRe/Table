use crate::compiler::parser2::vm2::pointers::{ConstantPointer, HeapPointer, InstructionPointer, LocalPointer, StackPointer};

#[derive(Debug, Clone, PartialEq)]
pub enum Bytecode {
    /// takes the value `distance = u32` and `stack.peek(distance)`
    DupAt(usize),
    /// dups the top of the stack.
    Dup,
    /// pops `value` off stack and does nothing with it.
    Pop,
    /// pops `condition` and jumps `if condition` to `InstructionPointer`
    JumpIf(InstructionPointer),
    /// pushes `constant_vars[u32]` onto stack
    LoadConstant(ConstantPointer),
    /// pushes `Heap[u32]` onto stack, doesn't work if it's a `String` or `Table`
    LoadHeap(HeapPointer),
    /// pushes `Heap.push(Table::new())` and pushes `Heap.len()-1` onto stack
    AllocTable,
    /// pushes `Heap.push(String::new())` and pushes `Heap.len()-1` onto stack
    AllocString,
    /// peeks `value` an pushes it onto heap and pushes `heap_index` onto stack
    AllocValue,
    /// pushes `local_vars.peek(usize)` onto stack
    PeekLocal(usize),
    /// pushes `local_vars[LocalPointer]` onto stack
    FindLocal(LocalPointer),
    /// peeks `value` and `local_vars.push(value)`
    PushLocal,
    /// `local_vars.pop()` and pushes onto stack
    PopLocal,
    /// pops `value` and `local_vars[LocalPointer] = value`
    SetLocal(LocalPointer),
    /// `local = local_vars[LocalPointer]` and then `heap.push(local)` and then pushes `heap.len()-1` onto stack
    AllocLocal(LocalPointer),
    /// pops `rhs` `lhs` and pushes `lhs + rhs` onto stack
    Add,
    /// pops `rhs` `lhs` and pushes `lhs == rhs` onto stack
    Eq,
    /// pops `value` and pushes `!value` onto stack
    Invert,
    /// pops `table_index` `heap_index` and pushes `Heap[heap_index][table_index]`
    GetTableNum,
    /// pops `str_index` `heap_index` and pushes `Heap[heap_index].get(Heap[str_index])`
    GetTableStr,
    /// peeks `value` `table_index` `heap_index` and `Heap[heap_index][table_index] = value`
    SetTableNum,
    /// peeks `value` `str_index` `heap_index` and `Heap[heap_index].get(Heap[str_index]) = value`
    SetTableStr,
    /// pops `value` peeks `heap_index` and `table = Heap[heap_index]` `table.push(value)` and pushes `table.len()-1` onto stack
    PushTableNum,
    /// pops `value` peeks `str_index` `heap_index` `table = Heap[heap_index]` `table.insert(str_index, value)` and pushes `table.len()-1` onto stack
    PushTableStr,
    /// pops `value` peeks `heap_index` and `string = Heap[heap_index]` `string.push(value)`
    PushChar,
}