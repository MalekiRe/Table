use std::fmt::{Display, Formatter};
use indexmap::IndexMap;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    Table(usize),
    Nil,
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Table(_) => unimplemented!(),
            Value::Nil => write!(f, "Nil"),
        }
    }
}
pub enum Table {
    Map(IndexMap<TableKey, Value>),
}
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum TableKey {
    Identifier(String, usize),
    NoIdentifier(usize),
}
impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}
impl Value {
    pub fn get_truthy(self) -> bool {
        match self {
            Value::Number(num) => num != 0,
            Value::Boolean(bool) => bool,
            Value::Nil => false,
            Value::Table(_) => unimplemented!(),
        }
    }
    pub fn get_number(self) -> Option<i64> {
        match self {
            Value::Number(num) => Some(num),
            Value::Boolean(_) => None,
            Value::Nil => None,
            Value::Table(_) => unimplemented!(),
        }
    }
}