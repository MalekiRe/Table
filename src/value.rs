use std::fmt::{Display, Formatter};
use crate::table::Table;

pub type TablePointer = usize;

#[derive(Clone, Copy)]
pub enum Value {
    Table(TablePointer),
    Int(i64),
    Float(f64),
    Boolean(bool),
    EmptyTable,
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
