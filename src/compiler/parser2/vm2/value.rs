use simple_error::SimpleError;
use crate::compiler::parser2::vm2::pointers::HeapPointer;
use crate::compiler::parser2::vm2::table::Table;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackValue {
    Nil,
    Char(char),
    Number(f32),
    Boolean(bool),
    String(HeapPointer),
    Table(HeapPointer),
}

#[derive(Debug, Clone)]
pub enum HeapValue {
    Nil,
    Char(char),
    Number(f32),
    Boolean(bool),
    String(String),
    Table(Table),
}

impl StackValue {
    pub fn try_to_string_pointer(self) -> Result<HeapPointer, SimpleError> {
        match self {
            StackValue::String(heap_pointer) => Ok(heap_pointer),
            _ => Err(SimpleError::new("not a string pointer")),
        }
    }
    pub fn try_to_table_pointer(self) -> Result<HeapPointer, SimpleError> {
        match self {
            StackValue::Table(heap_pointer) => Ok(heap_pointer),
            _ => Err(SimpleError::new("not a table pointer")),
        }
    }
    pub fn try_to_heap_value(&self) -> Result<HeapValue, SimpleError> {
        match self {
            StackValue::Nil => Ok(HeapValue::Nil),
            StackValue::Char(char) => Ok(HeapValue::Char(*char)),
            StackValue::Number(number) => Ok(HeapValue::Number(*number)),
            StackValue::Boolean(bool) => Ok(HeapValue::Boolean(*bool)),
            StackValue::String(_) => Err(SimpleError::new("is a string, not able to be turned into a heap value")),
            StackValue::Table(_) => Err(SimpleError::new("is a table, not able to be turned into a heap value")),
        }
    }
}

impl HeapValue {
    pub fn try_to_stack_value(&self) -> Result<StackValue, SimpleError> {
        match self {
            HeapValue::Nil => Ok(StackValue::Nil),
            HeapValue::Char(char) => Ok(StackValue::Char(*char)),
            HeapValue::Number(number) => Ok(StackValue::Number(*number)),
            HeapValue::Boolean(bool) => Ok(StackValue::Boolean(*bool)),
            HeapValue::String(_) => Err(SimpleError::new("is a string, not able to be turned into a stack value")),
            HeapValue::Table(_) => Err(SimpleError::new("is a table, not able to be turned into a stack value")),
        }
    }
}


