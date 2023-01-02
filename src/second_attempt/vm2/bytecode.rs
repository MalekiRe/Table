use std::mem::size_of;

pub const CONSTANT: u8 = 0x01;
pub const GET_LOCAL: u8 = 0x02;
pub const SET_LOCAL: u8 = 0x03;
pub const PEEK_LOCAL: u8 = 0x04;
pub const POP_LOCAL: u8 = 0x05;
pub const PUSH_LOCAL: u8 = 0x06;
pub const TEST_TRUTHY: u8 = 0x07;
pub const TEST_EQUAL: u8 = 0x08;
pub const TEST_LESS: u8 = 0x09;
pub const TEST_LESS_EQUAL: u8 = 0x0A;
pub const TEST_GREATER: u8 = 0x0B;
pub const TEST_GREATER_EQUAL: u8 = 0x0C;
pub const TEST_NOT: u8 = 0x0D;
pub const INJECT: u8 = 0x0E;
pub const JUMP: u8 = 0x0F;
pub const JUMP_IF: u8 = 0x10;
pub const ADD: u8 = 0x11;
pub const SUBTRACT: u8 = 0x12;
pub const MULTIPLY: u8 = 0x13;
pub const DIVIDE: u8 = 0x14;
pub const PRINT: u8 = 0x15;
pub const RETURN: u8 = 0x16;
pub const ALLOC_TABLE: u8 = 0x17;
pub const INSERT_INDEX_TABLE: u8 = 0x18;
pub const INSERT_STR_TABLE: u8 = 0x19;
pub const GET_INDEX_TABLE: u8 = 0x1A;
pub const GET_STR_TABLE: u8 = 0x1B;

pub enum Bytecode {
    Constant(usize),
    GetLocal(usize),
    SetLocal(usize),
    PeekLocal(usize),
    PopLocal,
    PushLocal,
    TestTruthy,
    TestEqual,
    TestLess,
    TestLessEqual,
    TestGreater,
    TestGreaterEqual,
    TestNot,
    Inject(usize),
    Jump(usize),
    JumpIf(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,
    Return,
    AllocTable, // pushes the address onto the stack,
    InsertIndexTable, // pops the usize index Value and the value to put *into* the table at that index.
    InsertStringTable, // pops the index into the String table for the key, and then pops value to put *into* the table at that index.
    GetIndexTable, // pops the table index, and the index *into* the table and pushes the value found.
    GetStringTable, // pops the table index and the index into the table string that indexes into the table for the value found.
}
fn represent(a: u8, val: usize) -> Vec<u8> {
    let mut vec = vec![a];
    vec.append(&mut convert_thing(val));
    vec
}
pub fn convert_thing(val: usize) -> Vec<u8> {
    unsafe {
        std::mem::transmute_copy::<usize, [u8; size_of::<usize>()]>(&val).to_vec()
    }
}
pub fn convert_back(val: &[u8]) -> usize {
    unsafe {
        std::mem::transmute_copy::<[u8; size_of::<usize>()], usize>(&val.try_into().unwrap())
    }
}
pub fn convert_bytecode_array(bytecode: Vec<Bytecode>) -> Vec<u8> {
    let mut ret_vec = vec![];
    for code in bytecode {
        ret_vec.append(&mut code.to_bytes());
    }
    ret_vec
}
impl Bytecode {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Bytecode::Constant(val) => represent(CONSTANT, val),
            Bytecode::GetLocal(val) => represent(GET_LOCAL, val),
            Bytecode::SetLocal(val) => represent(SET_LOCAL, val),
            Bytecode::PeekLocal(val) => represent(PEEK_LOCAL, val),
            Bytecode::PopLocal => vec![POP_LOCAL],
            Bytecode::PushLocal => vec![PUSH_LOCAL],
            Bytecode::TestTruthy => vec![TEST_TRUTHY],
            Bytecode::TestEqual => vec![TEST_EQUAL],
            Bytecode::TestLess => vec![TEST_LESS],
            Bytecode::TestLessEqual => vec![TEST_LESS_EQUAL],
            Bytecode::TestGreater => vec![TEST_GREATER],
            Bytecode::TestGreaterEqual => vec![TEST_GREATER_EQUAL],
            Bytecode::TestNot => vec![TEST_NOT],
            Bytecode::Inject(val) => represent(INJECT, val),
            Bytecode::Jump(val) => represent(JUMP, val),
            Bytecode::JumpIf(val) => represent(JUMP_IF, val),
            Bytecode::Add => vec![ADD],
            Bytecode::Subtract => vec![SUBTRACT],
            Bytecode::Multiply => vec![MULTIPLY],
            Bytecode::Divide => vec![DIVIDE],
            Bytecode::Print => vec![PRINT],
            Bytecode::Return => vec![RETURN],
            Bytecode::AllocTable => vec![ALLOC_TABLE],
            Bytecode::InsertIndexTable => vec![INSERT_INDEX_TABLE],
            Bytecode::InsertStringTable => vec![INSERT_STR_TABLE],
            Bytecode::GetIndexTable => vec![GET_INDEX_TABLE],
            Bytecode::GetStringTable => vec![GET_STR_TABLE],
        }
    }
}