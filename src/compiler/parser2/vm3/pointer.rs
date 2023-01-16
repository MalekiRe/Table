#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeapPointer {
    Chunk(ChunkPointer),
    Table(TablePointer),
    String(StringPointer),
    Number(NumberPointer),
    Boolean(BooleanPointer),
    Char(CharPointer),
}

#[derive(Debug, Clone)]
pub struct LocalDistance(pub usize);
#[derive(Debug, Clone)]
pub struct StackDistance(pub usize);
#[derive(Debug, Clone)]
pub struct ConstantPointer(pub usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChunkPointer(usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TablePointer(usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StringPointer(usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NumberPointer(pub usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BooleanPointer(pub usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CharPointer(pub usize);

macro_rules! usize_conversion {
    ($ident:ident) => {
        impl From<usize> for $ident {
            fn from(value: usize) -> Self {
                $ident(value)
            }
        }
        impl Into<usize> for $ident {
            fn into(self) -> usize {
                self.0
            }
        }
    }
}

usize_conversion!(ChunkPointer);
usize_conversion!(TablePointer);
usize_conversion!(StringPointer);
usize_conversion!(NumberPointer);
usize_conversion!(BooleanPointer);
usize_conversion!(CharPointer);

usize_conversion!(LocalDistance);
usize_conversion!(StackDistance);
usize_conversion!(ConstantPointer);

impl Into<usize> for HeapPointer {
    fn into(self) -> usize {
        match self {
            HeapPointer::Chunk(pointer) => pointer.into(),
            HeapPointer::Table(pointer) => pointer.into(),
            HeapPointer::String(pointer) => pointer.into(),
            HeapPointer::Number(pointer) => pointer.into(),
            HeapPointer::Boolean(pointer) => pointer.into(),
            HeapPointer::Char(pointer) => pointer.into(),
        }
    }
}