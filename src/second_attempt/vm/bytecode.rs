pub type TIdentifier = String;

#[derive(Clone)]
pub enum Bytecode {
    Return,
    Print,
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
    Jump(usize),
    JumpIf(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Copy, Clone, Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    Nil,
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
        }
    }
    pub fn get_number(self) -> Option<i64> {
        match self {
            Value::Number(num) => Some(num),
            Value::Boolean(_) => None,
            Value::Nil => None,
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
        if index >= N {
            return None;
        }
        Some(self.stack[index])
    }
    pub fn copy_head(&mut self) -> Option<>
    pub fn set(&mut self, index: usize, value: Value) -> Option<()> {
        if index >= N {
            return None;
        }
        self.stack[index] = value;
        Some(())
    }
    pub fn clear(&mut self) {
        self.len = 0;
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