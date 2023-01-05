use std::collections::HashMap;
use crate::{Bytecode, ErrorT, Exp, LetStatement, Value};
use crate::bytecode::Bytecode2;
use crate::bytecode::Bytecode2::{HeapTablePush, HeapTablePushWithKey, Jump, LoadConstant, LoadNumber, Pop, RegisterSet};
use crate::compiler::{ir, parser};
use crate::compiler::ir::{BStatement, ExpBlock, File, Statement, StatementBlock};
use crate::ir::{FnImport, IdentifierT, LiteralValue, TableKeyTemp, TableLiteral};
use crate::misc::VecTuple1;

pub struct IRCompiler {
    bytecode: Vec<Bytecode2>,
    constants: Vec<Value>,
    scope_holder: ScopeHolder,
    stack_size: usize,
}
#[derive(Clone)]
pub struct FnHeader {
    pub identifier: IdentifierT,
    pub args: Vec<IdentifierT>,
}
pub enum Variable {
    Local(usize),
    Table(usize),
}
pub struct Scope {
    variables: HashMap<IdentifierT, usize>,
    tables: HashMap<IdentifierT, usize>,
    imported_functions: Vec<IdentifierT>,
    functions: HashMap<IdentifierT, (usize, FnHeader)>
}
pub struct ScopeHolder(Vec<Scope>);
impl ScopeHolder {
    pub fn add_variable(&mut self, variable_name: IdentifierT, index: usize) {
        self.0.last_mut().unwrap().variables.insert(variable_name, index);
    }
    pub fn find_variable(&mut self, variable_name: IdentifierT) -> Option<usize> {
        let mut index = self.0.len()-1;
        while index > 0 {
            match self.0[index].variables.get(&variable_name) {
                None => (),
                Some(thing) => {
                    return Some(*thing)
                }
            }
        }
        None
    }
    pub fn find_function(&self, fn_name: IdentifierT) -> Option<&(usize, FnHeader)>{
        let mut index = self.0.len()-1;
        while index > 0 {
            match self.0[index].functions.get(&fn_name) {
                None => (),
                Some(thing) => return Some(thing)
            }
            index -= 1;
        }
        None
    }
    pub fn add_function(&mut self, fn_name: IdentifierT, thing: (usize, FnHeader)) {
        self.0.last_mut().unwrap().functions.insert(fn_name, thing);
    }
    pub fn add_imported_function(&mut self, fn_name: IdentifierT, thing: (usize, FnHeader)) {
        self.add_function(fn_name.clone(), thing);
        self.0.last_mut().unwrap().imported_functions.push(fn_name);
    }
    pub fn push(&mut self) {
        self.0.push(Scope::default())
    }
    pub fn pop(&mut self) {
        self.0.pop();
    }
}
impl Default for ScopeHolder {
    fn default() -> Self {
        Self {
            0: vec![Scope::default()]
        }
    }
}
impl Default for Scope {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            tables: Default::default(),
            imported_functions: Default::default(),
            functions: Default::default()
        }
    }
}
impl IRCompiler {
    pub fn compiler(file: File) -> (Vec<Bytecode2>, Vec<Value>) {
        let mut this = Self {
            bytecode: vec![],
            constants: vec![],
            scope_holder: Default::default(),
            stack_size: 0
        };
        this.compile_file(file);
        (this.bytecode, this.constants)
    }
    fn compile_file(&mut self, file: File) {
        match file {
            File::JustStatements(statements) => {
                match statements {
                    VecTuple1(first, rest) => {
                        self.statement(*first);
                        rest.into_iter().for_each(|s| self.statement(*s))
                    }
                }
            }
            File::StatementExp(statements, exp) => {
                statements.into_iter().for_each(|s| self.statement(*s));
                self.exp(*exp);
            }
            File::Empty => {}
        }
    }
    fn statement(&mut self, statement: Statement) {
        match statement {
            Statement::FnDec(_) => todo!(),
            Statement::FnImport(_) => todo!(),
            Statement::LetStatement(let_stmt) => self.let_statement(let_stmt),
            Statement::ExpStatement(_) => todo!(),
            Statement::StatementBlock(_) => todo!(),
            Statement::UnaryPostfixOperation(_) => todo!(),
        }
    }
    fn let_statement(&mut self, let_statement: LetStatement) {
        match let_statement {
            LetStatement { identifier, lhs } => {
                self.exp(*lhs);
                self.scope_holder.add_variable(identifier, self.stack_size);
                self.pop();
            }
        }
    }
    fn exp(&mut self, exp: Exp) {
        match exp {
            Exp::ExpBlock(_) => todo!(),
            Exp::LiteralValue(value) => self.literal_value(value),
            Exp::FnCall(_) => todo!(),
            Exp::TableOperation(_) => todo!(),
            Exp::Variable(_) => todo!(),
            Exp::UnaryPrefixOperation(_) => todo!(),
            Exp::BinaryOperation(_) => todo!()
        }
    }
    fn literal_value(&mut self, value: LiteralValue) {
        match value {
            LiteralValue::Decimal(decimal) => self.push_decimal(decimal),
            LiteralValue::Integer(integer) => self.push_integer(integer),
            LiteralValue::String(string) => self.push_string_literal(string),
            LiteralValue::Table(table) => self.push_table_literal(table),
            LiteralValue::Boolean(bool) => self.push_bool(bool),
        }
    }
    // fn fn_call_convention(&mut self, fn_identifier: IdentifierT) {
    //     let (ptr, header) = self.scope_holder.find_function(fn_identifier).unwrap().clone();
    //     self.byte_push(Jump(ptr));
    // }
    fn byte_push(&mut self, byte: Bytecode2) {
        match byte {
            Bytecode2::AllocHeap | LoadNumber(_) | LoadConstant(_) |
            Bytecode2::LoadHeapValue(_) | Bytecode2::LoadIndexHeapValue |
            Bytecode2::Peek(_) | Bytecode2::HeapTableGetIndex |
            Bytecode2::RegisterGet(_) | HeapTablePushWithKey => {
                self.stack_size += 1;
            }
            Bytecode2::HeapTablePush | RegisterSet(_) | Jump(_) |
            Bytecode2::JumpIf(_) | Bytecode2::HeapTableSetIndex => {
                //do nothing
            }
            Bytecode2::Pop => {
                self.stack_size -= 1;
            }
        }
        self.bytecode.push(byte)
    }
    fn push_table_literal(&mut self, table_literal: TableLiteral) {
        self.alloc_heap();
        for thing in table_literal.into_iter() {
            //TODO add string keys
            match thing {
                TableKeyTemp { ident, exp } => {
                    self.exp(*exp);
                    match ident {
                        None => {
                            self.heap_table_push();
                        }
                        Some(ident) => {
                            self.push_string_literal(ident);
                            self.heap_table_push_with_key()
                        }
                    }
                }
            }
            self.pop();
        }
    }
    fn push_string_literal(&mut self, string: String) {
        self.alloc_heap();
        for char in string.chars() {
            self.push_integer(char as isize);
            self.heap_table_push();
            self.pop();
        }
    }
    fn push_decimal(&mut self, decimal: f64) {
        let value = Value::Float(decimal);
        let index = match self.constants.iter().position(|&v| v == value) {
            None => {
                self.constants.push(value);
                self.constants.len()-1
            }
            Some(index) => index,
        };
        self.load_constant(index);
    }
    fn push_integer(&mut self, integer: isize) {
        self.load_number(integer);
    }
    fn push_bool(&mut self, bool: bool) {
        let value = Value::Boolean(bool);
        let index = match self.constants.iter().position(|&v| v == value) {
            None => {
                self.constants.push(value);
                self.constants.len()-1
            }
            Some(index) => index,
        };
        self.load_constant(index);
    }
    fn heap_table_push(&mut self) {
        self.byte_push(HeapTablePush);
    }
    fn heap_table_push_with_key(&mut self) {
        self.byte_push(HeapTablePushWithKey);
    }
    fn pop(&mut self) {
        self.byte_push(Pop);
    }
    fn load_constant(&mut self, index: usize) {
        self.byte_push(LoadConstant(index))
    }
    fn load_number(&mut self, number: isize) {
        self.byte_push(LoadNumber(number as usize))
    }
    fn alloc_heap(&mut self) {
        self.byte_push(Bytecode2::AllocHeap);
    }
}
pub type TablePointer = usize;
#[cfg(test)]
mod test {
    use crate::bytecode::Bytecode2;
    use crate::ir::ir_bytecode_compiler::{IRCompiler, parse_file};
    use crate::Value;
    use crate::virtual_machine::chunk::Chunk2;
    use crate::virtual_machine::table::{Table, TableKey};
    use crate::virtual_machine::value::HeapValue;
    use crate::virtual_machine::vm::Vm2;

    #[test]
    fn literal_exp() {
        let file = parse_file("1").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(constants, vec![]);
        assert_eq!(bytecode, vec![Bytecode2::LoadNumber(1)]);
        let file = parse_file("1.0").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(constants, vec![Value::Float(1.0)]);
        assert_eq!(bytecode, vec![Bytecode2::LoadConstant(0)]);
        let file = parse_file("false").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(constants, vec![Value::Boolean(false)]);
        assert_eq!(bytecode, vec![Bytecode2::LoadConstant(0)]);
        let file = parse_file("\"hi\"").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(bytecode, vec![
            Bytecode2::AllocHeap,
            Bytecode2::LoadNumber('h' as usize),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
            Bytecode2::LoadNumber('i' as usize),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
        ])
    }
    #[test]
    fn literal_table() {
        let file = parse_file("[1, a: false, 3.0]").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(constants, vec![Value::Boolean(false), Value::Float(3.0)]);
        assert_eq!(bytecode, vec![
            Bytecode2::AllocHeap,
            Bytecode2::LoadNumber(1),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
            Bytecode2::LoadConstant(0),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
            Bytecode2::LoadConstant(1),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
        ]);
        let chunk = Chunk2::new(bytecode, constants);
        let mut vm = Vm2::new();
        vm.load(chunk);
        vm.run();
        //assert_eq!(vm.heap.pop().unwrap(), HeapValue::Table(Table{ inner: vec![] }))
    }
    fn literal_table_array() {
        let file = parse_file("[1, false, 3.0]").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        assert_eq!(constants, vec![Value::Boolean(false), Value::Float(3.0)]);
        assert_eq!(bytecode, vec![
            Bytecode2::AllocHeap,
            Bytecode2::LoadNumber(1),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
            Bytecode2::LoadConstant(0),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
            Bytecode2::LoadConstant(1),
            Bytecode2::HeapTablePush,
            Bytecode2::Pop,
        ]);
        let chunk = Chunk2::new(bytecode, constants);
        let mut vm = Vm2::new();
        vm.load(chunk);
        vm.run();
        let test_table = Table {
            inner: vec![
                TableKey::NoStr(Value::Int(1)),
                TableKey::NoStr(Value::Boolean(false)),
                TableKey::NoStr(Value::Float(3.0))
            ]
        };
        assert_eq!(vm.heap.pop().unwrap(), HeapValue::Table(test_table))
    }
    #[test]
    fn let_statement() {
        let file = parse_file("let x = 1;").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);

    }
    #[test]
    fn vm_literal() {
        let file = parse_file("1").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        let chunk = Chunk2::new(bytecode, constants);
        let mut vm = Vm2::new();
        vm.load(chunk);
        vm.run();
        assert_eq!(vm.pop(), Value::Int(1));
        let file = parse_file("true").unwrap();
        let (bytecode, constants) = IRCompiler::compiler(file);
        let chunk = Chunk2::new(bytecode, constants);
        let mut vm = Vm2::new();
        vm.load(chunk);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(true));
    }
}
pub fn parse_file(file: &str) -> Option<ir::File> {
    let file_holder = crate::FileHolder::from(file.to_string().clone());
    let (ir, errors) = crate::compiler::parser::parse_block(file.to_string());
    assert_eq!(errors.len(), 0, "{:?}", get_errors_display(errors, file_holder));
    assert!(ir.is_some());
    ir
}
pub fn get_errors_display(errors: Vec<ErrorT>, mut file_holder: crate::FileHolder) -> String {
    let mut str = Vec::new();
    for error in errors {
        error.write(&mut file_holder, std::io::stderr());
    }
    String::from_utf8(str).unwrap()
}