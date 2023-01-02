use indexmap::IndexMap;
use crate::second_attempt::vm::bytecode::Bytecode::{Constant, PopLocal, PushLocal, Return, TestTruthy};

pub type TIdentifier = String;

#[derive(Clone)]
pub enum Bytecode {
    Return,
    Constant(Value),
    GetLocal(usize),
    SetLocal(usize),
    Copy,
    PushLocal,
    PopLocal,
    TestTruthy,
    TestEqual,
    TestLess,
    TestLessEqual,
    TestGreater,
    TestGreaterEqual,
    TestNot,
    Inject,
    Jump(usize),
    JumpIf(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,
}
/*#[derive(Clone)]
pub enum SingleByte {
    Return = 0,
    Copy = 1,
    PushLocal = 2,
    PopLocal = 3,
    TestTruthy = 4,
    TestEqual = 5,
    TestLess = 6,
    TestLessEqual = 7,
    TestGreater = 8,
    TestGreaterEqual = 9,
    TestNot = 10,
    Add = 11,
    Subtract = 12,
    Multiply = 13,
    Divide = 14,
    Print = 15,
}
pub enum USizeByte {
    GetLocal(usize),
    SetLocal(usize),
    Jump(usize),
    JumpIf(usize),
}
#[derive(Copy)]
pub enum ValueByte {
    Constant(Value),
}
 */
#[derive(Copy, Clone, Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    Table(usize),
    Nil,
}
pub type TableStack = Vec<Table>;
// TODO: make a table an enum that has a key value map but also an array that's just a slice
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