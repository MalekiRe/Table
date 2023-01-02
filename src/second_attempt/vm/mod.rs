use std::collections::VecDeque;
use crate::second_attempt::vm::bytecode::{Bytecode, Stack, Value};
use crate::second_attempt::vm::bytecode::Bytecode::{Constant, GetLocal, Jump, JumpIf, Print, Return, SetLocal, PushLocal, PopLocal, TestEqual, TestTruthy};
use crate::second_attempt::vm::bytecode::Value::{Boolean, Number};

pub mod bytecode;

pub struct Vm {
    bytecode: Vec<Bytecode>,
    current_index: usize,
    local_stack: Stack<256>,
    eval_stack: Stack<256>,
}
impl Vm {
    pub fn from(bytecode: Vec<Bytecode>) -> Self {
        Self {
            bytecode,
            current_index: 0,
            local_stack: Default::default(),
            eval_stack: Default::default()
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.read_byte() {
                Return => {}
                Constant(value) => {
                    self.eval_stack.push(value).unwrap()
                }
                Print => {
                    println!("{:#?}", self.eval_stack.pop().unwrap());
                }
                GetLocal(index) => {
                    let val = self.local_stack.peek(index).unwrap();
                    self.eval_stack.push(val).unwrap();
                }
                SetLocal(index) => {
                    let val = self.eval_stack.pop().unwrap();
                    self.local_stack.set(index, val).unwrap();
                }
                PushLocal => {
                    let val = self.eval_stack.pop().unwrap();
                    self.local_stack.push(val).unwrap();
                }
                PopLocal => {
                    let val = self.local_stack.pop().unwrap();
                    self.eval_stack.push(val).unwrap();
                }
                Jump(position) => {
                    self.current_index = position;
                }
                JumpIf(position) => {
                    if self.eval_stack.pop().unwrap().get_truthy() {
                        self.current_index = position;
                    }
                }
                TestEqual => {
                    let first = self.eval_stack.pop().unwrap();
                    let second = self.eval_stack.pop().unwrap();
                    //TODO allow matching on certain things like boolean with none types
                    if !matches!(first, second) {
                        panic!("values aren't of the same type");
                    }
                    let ret = match first {
                        Number(val_1) => {
                            match second {
                                Number(val_2) => val_1 == val_2,
                                _=> panic!("values aren't of the same type: {}", val_1)
                            }
                        }
                        Boolean(val_1) => {
                            match second {
                                Boolean(val_2) => val_2 == val_1,
                                _=> panic!("values aren't of the same type: {}", val_1)
                            }
                        }
                        Value::Nil => {
                            match second {
                                Value::Nil => true,
                                _=>  panic!("values aren't of the same type: Nil")
                            }
                        }
                        Value::Table(_) => unimplemented!(),
                    };
                    self.eval_stack.push(Value::Boolean(ret)).unwrap();
                }
                TestTruthy => {
                    let val = self.eval_stack.pop().unwrap().get_truthy();
                    self.eval_stack.push(Value::Boolean(val));
                }
                Bytecode::TestLess => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Boolean(val_2 < val_1));
                }
                Bytecode::TestLessEqual => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Boolean(val_2 <= val_1));
                }
                Bytecode::TestGreater => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Boolean(val_2 > val_1));
                }
                Bytecode::TestGreaterEqual => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Boolean(val_2 >= val_1));
                }
                Bytecode::TestNot => {
                    let val_1 = self.eval_stack.pop().unwrap().get_truthy();
                    self.eval_stack.push(Boolean(!val_1));
                }
                Bytecode::Add => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Number(val_2 + val_1));
                }
                Bytecode::Subtract => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Number(val_2 - val_1));
                }
                Bytecode::Multiply => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Number(val_2 * val_1));
                }
                Bytecode::Divide => {
                    let val_1 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    let val_2 = self.eval_stack.pop().unwrap().get_number().unwrap();
                    self.eval_stack.push(Number(val_2 / val_1));
                }
                Bytecode::Copy => {
                    let val = self.eval_stack.peek(0).unwrap();
                    self.eval_stack.push(val);
                }
                Bytecode::Inject => {
                    unimplemented!()
                }
            }
            if self.current_index == self.bytecode.len() {
                return;
            }
        }
    }
    pub fn read_byte(&mut self) -> Bytecode {
        let bytecode = self.bytecode.get(self.current_index).unwrap();
        self.current_index += 1;
        bytecode.clone()
    }
    pub fn clear_stack(&mut self) {
        self.local_stack.clear();
        self.eval_stack.clear();
    }
}
pub fn test_vm() {
    println!("testing vm!");
    //let mut vm = Vm::from(vec![Constant(Number(1)), PushLocal, Constant(Number(3)), Constant(Number(4)), PushLocal, GetLocal(0), Print, Print, PopLocal, Print]);
    let mut vm = Vm::from(vec![
        Constant(Number(30)),
        Constant(Number(1)),
        TestEqual,
        Print,
    ]);
    vm.run();
    println!("end of vm test!");
}