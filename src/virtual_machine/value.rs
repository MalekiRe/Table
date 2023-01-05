use std::fmt::{Display, Formatter};
use crate::virtual_machine::table::Table;

pub type TablePointer = usize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Table(TablePointer),
    Int(i64),
    Float(f64),
    Boolean(bool),
    EmptyTable,
}
#[derive(Debug, PartialEq)]
pub enum HeapValue {
    Value(Value),
    Table(Table)
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Table(_) => unimplemented!(),
            Value::Int(int) => write!(f, "{}", int),
            Value::Float(float) => write!(f, "{}", float),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::EmptyTable => write!(f, "Empty Table"),
        }
    }
}
