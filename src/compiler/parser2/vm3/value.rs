use simple_error::SimpleError;
use crate::compiler::parser2::vm3::chunk::Chunk;
use crate::compiler::parser2::vm3::pointer::{ChunkPointer, HeapPointer};
use crate::compiler::parser2::vm3::table::Table;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StackValue {
    Nil,
    Number(f32),
    Char(char),
    Boolean(bool),
    HeapPointer(HeapPointer),
}

#[derive(Debug)]
pub enum HeapValue {
    Number(f32),
    Char(char),
    Boolean(bool),
    Chunk(Chunk),
    Table(Table),
    String(String),
}

impl TryFrom<StackValue> for HeapPointer {
    type Error = SimpleError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::HeapPointer(heap_pointer) => Ok(heap_pointer),
            this => Err(SimpleError::new(format!("is not heap pointer is: {:?}", this)))
        }
    }
}

impl TryFrom<&HeapValue> for StackValue {
    type Error = SimpleError;

    fn try_from(value: &HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::Number(number) => Ok(StackValue::Number(*number)),
            HeapValue::Char(char) => Ok(StackValue::Char(*char)),
            HeapValue::Boolean(bool) => Ok(StackValue::Boolean(*bool)),
            this => Err(SimpleError::new(format!("is not a stack convertable value is: {:?}", this)))
        }
    }
}

impl TryFrom<StackValue> for ChunkPointer {
    type Error = SimpleError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::HeapPointer(heap_pointer) => {
                match heap_pointer {
                    HeapPointer::Chunk(chunk_pointer) => Ok(chunk_pointer),
                    this => Err(SimpleError::new("conversion failure")),
                }
            }
            this => Err(SimpleError::new("conversion failure")),
        }
    }
}