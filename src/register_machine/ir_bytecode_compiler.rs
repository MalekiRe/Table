use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::default::Default;
use crate::compiler::parser::literal_value;
use crate::{Exp, LetStatement};
use crate::ir::{BinaryOp, BinaryOperation, File, IdentifierT, LiteralValue, MathOp, Statement};
use crate::ir::ir_bytecode_compiler::FnHeader;
use crate::register_machine::stack_value::StackValue;
use crate::register_machine::vm::{Bytecode, Chunk};
use crate::register_machine::vm::Bytecode::{AddPop, LoadConstant};

pub struct Scope {
    variables: HashMap<IdentifierT, Location>,
}
#[derive(Debug, Clone, Copy)]
pub enum Location {
    Stack(usize),
    Heap(usize),
    Constant(usize),
}
pub struct ScopeHolder(Vec<Scope>);
impl ScopeHolder {
    pub fn add_variable(&mut self, variable_name: IdentifierT, location: Location) {
        self.0.last_mut().unwrap().variables.insert(variable_name, location);
    }
    pub fn find_variable(&mut self, variable_name: IdentifierT) -> Option<Location> {
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
        }
    }
}
pub struct IRCompiler {
    bytecode: Vec<Bytecode>,
    consts: Vec<StackValue>,
    scope_holder: ScopeHolder,
}
impl IRCompiler {
    pub fn compile(file: File) -> Chunk {
        let mut this = Self {
            bytecode: vec![],
            consts: Default::default(),
            scope_holder: Default::default()
        };
        this.file(file);
        match this {
            IRCompiler { bytecode, consts, scope_holder } => {
                Chunk::from(bytecode, consts)
            }
        }
    }
    pub fn file(&mut self, file: File) {
        match file {
            File::JustStatements(_) => {}
            File::StatementExp(statements, exp) => {
                for statement in statements {
                    self.statement(*statement);
                }
                self.exp(*exp);
            }
            File::Empty => {}
        }
    }
    pub fn statement(&mut self, statement: Statement) {
        match statement {
            Statement::FnDec(_) => todo!(),
            Statement::FnImport(_) => todo!(),
            Statement::LetStatement(let_statement) => self.let_statement(let_statement),
            Statement::ExpStatement(_) => todo!(),
            Statement::StatementBlock(_) => todo!(),
            Statement::UnaryPostfixOperation(_) => todo!(),
        }
    }
    pub fn let_statement(&mut self, let_statement: LetStatement) {
        match let_statement {
            LetStatement { identifier, lhs } => {
                self.exp(*lhs);
                self.bytecode.push(Bytecode::PushLocal);
                self.bytecode.pop();
               //TODO dot his self.scope_holder.add_variable(identifier, Location::Stack(0));
            }
        }
    }
    pub fn exp(&mut self, exp: Exp) {
        match exp {
            Exp::ExpBlock(_) => todo!(),
            Exp::LiteralValue(value) => self.literal_value(value),
            Exp::FnCall(_) => todo!(),
            Exp::TableOperation(_) => todo!(),
            Exp::Variable(_) => todo!(),
            Exp::UnaryPrefixOperation(_) => todo!(),
            Exp::BinaryOperation(binary_operation) => self.binary_operation(binary_operation),
        }
    }
    pub fn binary_operation(&mut self, binary_operation: BinaryOperation) {
        match binary_operation {
            BinaryOperation { lhs, op, rhs } => {
                self.exp(*lhs);
                self.exp(*rhs);
                match op {
                    BinaryOp::Math(math) => match math {
                        MathOp::Add => {
                            self.bytecode.push(AddPop);
                        }
                        MathOp::Subtract => todo!(),
                        MathOp::Multiply => todo!(),
                        MathOp::Divide => todo!(),
                        MathOp::Modulo => todo!(),
                        MathOp::AddEqual => todo!(),
                        MathOp::MinusEqual => todo!(),
                        MathOp::MultiplyEqual => todo!(),
                        MathOp::DivideEqual => todo!(),
                        MathOp::ModuloEqual => todo!(),
                    }
                    BinaryOp::Equality(_) => todo!(),
                }
            }
        }
    }
    pub fn literal_value(&mut self, literal_value: LiteralValue) {
        let stack_value = match literal_value {
            LiteralValue::Decimal(decimal) => StackValue::Number(decimal as f32),
            LiteralValue::Integer(integer) => StackValue::Number(integer as f32),
            LiteralValue::String(string) => todo!(),
            LiteralValue::Table(table_literal) => todo!(),
            LiteralValue::Boolean(boolean) => StackValue::Boolean(boolean),
        };
        if !self.consts.contains(&stack_value) {
            self.consts.push(stack_value);
        }
        self.bytecode.push(LoadConstant((self.consts.iter().position(|&v| v == stack_value)).unwrap() as u32))
    }
}

#[cfg(test)]
mod test {
    use crate::ir::ir_bytecode_compiler::parse_file;
    use crate::register_machine::ir_bytecode_compiler::IRCompiler;
    use crate::register_machine::stack_value::StackValue;
    use crate::register_machine::vm::Vm;

    #[test]
    fn first() {
        let mut vm = Vm::new();
        let file = parse_file("1").unwrap();
        let chunk = IRCompiler::compile(file);
        vm.load(chunk);
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(1.0)])
    }
    #[test]
    fn second() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("1+2").unwrap()));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(3.0)])
    }
}