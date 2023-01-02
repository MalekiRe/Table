mod misc;
mod bytecode;

use std::mem::size_of;
use misc::Value;
use crate::second_attempt::vm2::bytecode::Bytecode::{Constant, Print};
use crate::second_attempt::vm2::bytecode::{convert_back, convert_bytecode_array};
use crate::second_attempt::vm2::misc::Table;

pub fn test_vm() {
    let instructions = vec![
        Constant(0),
        Print,
    ];
    let constants = vec![
        Value::Number(1)
    ];
    let mut vm = Vm::new(convert_bytecode_array(instructions), constants);
    vm.run();
}
struct Vm {
    locals: Stack<256>,
    eval: Stack<256>,
    tables: Vec<Table>,
    constants: Vec<Value>,
    instructions: Vec<u8>,
}
impl Vm {
    pub fn new(instructions: Vec<u8>, constants: Vec<Value>) -> Self {
        Self {
            locals: Stack::default(),
            eval: Stack::default(),
            tables: vec![],
            constants,
            instructions,
        }
    }
    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.instructions.len() {
            ip += 1;
            match *self.instructions.get(ip-1).unwrap() {
                bytecode::CONSTANT => {
                    let val = convert_back(&self.instructions.as_slice()[ip..ip+8]);
                    let val = *self.constants.get(val).unwrap();
                    self.eval.push(val);
                }
                bytecode::PRINT => {
                    let val = self.eval.pop().unwrap();
                    println!("{}", val);
                }
                _ => {}
            }
        }
    }
}
impl Default for Vm {
    fn default() -> Self {
        Self {
            locals: Stack::default(),
            eval: Stack::default(),
            tables: vec![],
            constants: vec![],
            instructions: vec![],
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