use simple_error::SimpleError;
use crate::register_machine::vm::{HeapValue, Table};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StackValue {
    Number(f32),
    String(u32),
    Table(u32),
    Boolean(bool),
    Nil,
}

impl StackValue {
    pub fn try_to_table_index(self) -> Result<usize, SimpleError> {
        match self {
            StackValue::Table(index) => Ok(index as usize),
            _ => Err(SimpleError::new("not a table"))
        }
    }
    pub fn try_to_str_index(self) -> Result<usize, SimpleError> {
        match self {
            StackValue::String(index) => Ok(index as usize),
            _ => Err(SimpleError::new("not a string")),
        }
    }
}

impl<'a> TryFrom<&'a HeapValue> for &'a String {
    type Error = SimpleError;

    fn try_from(value: &'a HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::String(string) => Ok(string),
            _ => Err(SimpleError::new("is not a string"))
        }
    }
}

impl<'a> TryFrom<&'a HeapValue> for &'a Table {
    type Error = SimpleError;

    fn try_from(value: &'a HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::Table(table) => Ok(table),
            _ => Err(SimpleError::new("error")),
        }
    }
}

impl<'a> TryFrom<&'a mut HeapValue> for &'a mut Table {
    type Error = SimpleError;

    fn try_from(value: &'a mut HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::Table(table) => Ok(table),
            _ => Err(SimpleError::new("error")),
        }
    }
}

impl<'a> TryFrom<&'a mut HeapValue> for &'a mut String {
    type Error = SimpleError;

    fn try_from(value: &'a mut HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::String(string) => Ok(string),
            _ => Err(SimpleError::new("error")),
        }
    }
}

impl TryFrom<f32> for StackValue {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(StackValue::Number(value))
    }
}

impl TryFrom<usize> for StackValue {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(StackValue::Number(value as f32))
    }
}

impl TryFrom<u32> for StackValue {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(StackValue::Number(value as f32))
    }
}

impl TryFrom<bool> for StackValue {
    type Error = ();

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(StackValue::Boolean(value))
    }
}


impl TryFrom<StackValue> for f32 {
    type Error = SimpleError;
    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Number(number) => Ok(number),
            _ => Err(SimpleError::new(format!("not a number: {:?}", value)))
        }
    }
}

impl TryFrom<StackValue> for usize {
    type Error = SimpleError;
    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Number(number) => Ok(number as usize),
            _ => Err(SimpleError::new(format!("not a number: {:?}", value)))
        }
    }
}

impl TryFrom<StackValue> for u32 {
    type Error = SimpleError;
    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Number(number) => Ok(number as u32),
            _ => Err(SimpleError::new(format!("not a number: {:?}", value)))
        }
    }
}

impl TryFrom<StackValue> for bool {
    type Error = SimpleError;
    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Boolean(bool) => Ok(bool),
            _ => Err(SimpleError::new(format!("not a bool: {:#?}", value)))
        }
    }
}

impl TryFrom<StackValue> for HeapValue {
    type Error = SimpleError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Number(number) => Ok(HeapValue::Number(number)),
            StackValue::Boolean(boolean) => Ok(HeapValue::Boolean(boolean)),
            StackValue::Nil => Ok(HeapValue::Nil),
            StackValue::String(_) |
            StackValue::Table(_) => Err(SimpleError::new("can't do this"))
        }
    }
}

impl TryFrom<HeapValue> for StackValue {
    type Error = SimpleError;

    fn try_from(value: HeapValue) -> Result<Self, Self::Error> {
        match value {
            HeapValue::Number(number) => Ok(StackValue::Number(number)),
            HeapValue::Boolean(boolean) => Ok(StackValue::Boolean(boolean)),
            HeapValue::Nil => Ok(StackValue::Nil),
            HeapValue::String(_) |
            HeapValue::Table(_) => Err(SimpleError::new("can't do this"))
        }
    }
}

impl TryFrom<StackValue> for char {
    type Error = SimpleError;
    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Number(number) => Ok(char::from_u32(number as u32).unwrap()),
            _ => Err(SimpleError::new("not a char"))
        }
    }
}