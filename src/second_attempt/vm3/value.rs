use std::fmt::{Display, Formatter};

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
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum TableKey {
    Identifier(String, Option<usize>),
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
pub struct Stack<const N: usize> {
    stack: [Value; N],
    len: usize,
}
impl<const N: usize> Stack<N> {
    pub fn pop(&mut self) -> Option<Value> {
        if self.len == 0 {
            return None;
        }
        let val = self.stack[self.len-1];
        self.len -= 1;
        Some(val)
    }
    pub fn push(&mut self, value: Value) -> Option<()> {
        if self.len == N {
            return None;
        }
        self.stack[self.len] = value;
        self.len += 1;
        Some(())
    }
    pub fn peek(&mut self, index: usize) -> Option<Value> {
        Some(self.stack[self.compute_distance(index)?])
    }
    pub fn set(&mut self, index: usize, value: Value) -> Option<()> {
        self.stack[self.compute_distance(index)?] = value;
        Some(())
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    fn compute_distance(&self, index: usize) -> Option<usize> {
        if self.len - index < 0 {
            return None;
        }
        Some(self.len-index)
    }
}
impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        Self {
            stack: [Default::default(); N],
            len: 0
        }
    }
}