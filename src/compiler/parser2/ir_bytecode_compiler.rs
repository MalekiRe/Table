use std::collections::HashMap;
use crate::compiler::ir::{IdentifierT};
use crate::compiler::parser2::parser::parse;
use crate::compiler::parser2::parsing_ir::{BStatement, Exp, ExpBlock, File, LetStatement, Literal, ReassignStatement, Statement, StatementBlock, Variable};
use crate::compiler::parser2::vm2::bytecode::Bytecode;
use crate::compiler::parser2::vm2::chunk::Chunk;
use crate::compiler::parser2::vm2::pointers::{ConstantPointer, HeapPointer, LocalPointer};
use crate::compiler::parser2::vm2::value::StackValue;


#[derive(Debug, Copy, Clone)]
pub enum Location {
    Heap(HeapPointer),
    Local(LocalPointer),
}

pub struct Scope {
    local_variables: HashMap<IdentifierT, LocalPointer>,
    heap_variables: HashMap<IdentifierT, HeapPointer>,
}

pub struct ScopeHolder {
    local_size: usize,
    heap_size: usize,
    scopes: Vec<Scope>
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            local_variables: Default::default(),
            heap_variables: Default::default()
        }
    }
}

impl Default for ScopeHolder {
    fn default() -> Self {
        Self {
            local_size: 0,
            heap_size: 0,
            scopes: vec![Scope::default()]
        }
    }
}

impl ScopeHolder {
    pub fn push_local_var(&mut self, identifier: IdentifierT) -> usize {
        let location = Location::Local(LocalPointer(self.local_size));
        self.add_var(identifier, location);
        match location {
            Location::Heap(a) => {a.0}
            Location::Local(a) => {a.0}
        }
    }
    fn add_var(&mut self, identifier: IdentifierT, location: Location) {
        let last = self.scopes.last_mut().unwrap();
        match location {
            Location::Heap(heap_pointer) => {
                last.heap_variables.insert(identifier, heap_pointer);
                self.heap_size += 1;
            }
            Location::Local(local_pointer) => {
                last.local_variables.insert(identifier, local_pointer);
                self.local_size += 1;
            }
        }
    }
    pub fn find_var(&self, identifier: IdentifierT) -> Option<Location> {
        let mut scope_index = self.scopes.len();
        loop {
            if scope_index == 0 { break; }
            scope_index -= 1;
            match self.scopes[scope_index].local_variables.get(identifier.as_str()) {
                None => {}
                Some(location) => {
                    return Some(Location::Local(*location))
                }
            }
            match self.scopes[scope_index].heap_variables.get(identifier.as_str()) {
                None => {}
                Some(location) => {
                    return Some(Location::Heap(*location))
                }
            }
        }
        None
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }
    pub fn pop_scope(&mut self) {self.scopes.pop().unwrap();}
}

pub struct IRCompiler {
    bytecode: Vec<Bytecode>,
    constants: Vec<StackValue>,
    scope_holder: ScopeHolder,
}
impl IRCompiler {
    fn new() -> Self {
        Self {
            bytecode: vec![],
            constants: vec![],
            scope_holder: Default::default()
        }
    }
    pub fn compile_string(code: &str) -> Chunk {
        let mut this = Self::new();
        this.compile_file(parse(code).unwrap())
    }
    pub fn compile_file(mut self, file: File) -> Chunk {
        self.file(file);
        match self {
            IRCompiler { bytecode, constants, scope_holder } => {
                Chunk::from(bytecode, constants)
            }
        }
    }
    pub fn add_local_var(&mut self, identifier: IdentifierT) -> usize {
        self.scope_holder.push_local_var(identifier)
    }
    pub fn add_constant(&mut self, stack_value: StackValue) -> ConstantPointer {
        //TODO add caching so we don't add constants we already have
        let ptr = ConstantPointer(self.constants.len());
        self.bytecode.push(Bytecode::LoadConstant(ptr));
        self.constants.push(stack_value);
        ptr
    }

    pub fn file(&mut self, file: File) {
        match file {
            File::StatementsExp(b_statements, b_exp) => {
                self.b_statements(b_statements);
                self.exp(*b_exp);
            }
            File::Statements(b_statements) => {
                self.b_statements(b_statements);
            }
        }
    }
    pub fn b_statements(&mut self, b_statements: Vec<BStatement>) {
        for b_statement in b_statements {
            self.statement(*b_statement);
        }
    }

    pub fn statement(&mut self, statement: Statement) {
        match statement {
            Statement::StatementBlock(statement_block) => self.statement_block(statement_block),
            Statement::LetStatement(let_statement) => self.let_statement(let_statement),
            Statement::ReassignStatement(reassign_statement) => self.reassign_statement(reassign_statement),
            Statement::BreakStatement(_) => todo!(),
            Statement::IfStatement(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
            Statement::ExpStatement(_) => todo!(),
            Statement::MacroCall(_) => todo!(),
        }
    }
    pub fn reassign_statement(&mut self, reassign_statement: ReassignStatement) {
        match reassign_statement {
            ReassignStatement::SingleVarAssign(identifier, exp) => {
                let identifier = match identifier {
                    Variable::Identifier(identifier) => identifier,
                    Variable::TableIndexing(_) => todo!(),
                    Variable::TableFieldAccess(_) => todo!(),
                };
                let location = self.scope_holder.find_var(identifier).unwrap();
                match location {
                    Location::Heap(_) => todo!(),
                    Location::Local(local_pointer) => {
                        self.exp(*exp);
                        self.bytecode.push(Bytecode::SetLocal(local_pointer));
                    }
                }
            }
            ReassignStatement::Table(_) => todo!(),
            ReassignStatement::UniqueIdentTable(_) => todo!(),
        }
    }
    pub fn statement_block(&mut self, statement_block: StatementBlock) {
        match statement_block {
            StatementBlock { statements } => {
                self.b_statements(statements)
            }
        }
    }
    pub fn let_statement(&mut self, let_statement: LetStatement) {
        match let_statement {
            LetStatement::Uninitialized(identifier) => {
                self.add_local_var(identifier);
                self.add_constant(StackValue::Nil);
                self.bytecode.push(Bytecode::PushLocal);
                self.bytecode.push(Bytecode::Pop);
            }
            LetStatement::SingleAssign(identifier, b_exp) => {
                self.add_local_var(identifier);
                self.exp(*b_exp);
                self.bytecode.push(Bytecode::PushLocal);
                self.bytecode.push(Bytecode::Pop);
            },
            LetStatement::Table(_) => todo!(),
            LetStatement::UniqueIdentTable(_) => todo!(),
        }
    }

    pub fn exp(&mut self, exp: Exp) {
        match exp {
            Exp::BinaryExp(_) => todo!(),
            Exp::ExpBlock(exp_block) => self.exp_block(exp_block),
            Exp::FnCall(_) => todo!(),
            Exp::FnDec(_) => todo!(),
            Exp::TableExp(_) => todo!(),
            Exp::ControlFlowExp(_) => todo!(),
            Exp::RangeCreation(_) => todo!(),
            Exp::MacroCall(_) => todo!(),
            Exp::LiteralCode(_) => todo!(),
            Exp::Literal(literal) => self.literal(literal),
            Exp::VariableIdentifier(variable_identifier) => self.variable_identifier(variable_identifier),
        }
    }
    pub fn exp_block(&mut self, exp_block: ExpBlock) {
        self.scope_holder.push_scope();
        match exp_block {
            ExpBlock { statements, exp } => {
                self.b_statements(statements);
                self.exp(*exp);
            }
        }
        self.scope_holder.pop_scope();
    }
    pub fn variable_identifier(&mut self, variable_identifier: IdentifierT) {
        let location = self.scope_holder.find_var(variable_identifier).unwrap();
        match location {
            Location::Heap(_) => todo!(),
            Location::Local(local_pointer) => {
                self.bytecode.push(Bytecode::FindLocal(local_pointer))
            }
        }
    }
    pub fn literal(&mut self, literal: Literal) {
        match literal {
            Literal::Char(char) => self.add_constant(StackValue::Char(char)),
            Literal::Number(number) => self.add_constant(StackValue::Number(number)),
            Literal::Boolean(boolean) => self.add_constant(StackValue::Boolean(boolean)),
            Literal::String(_) => todo!(),
            Literal::TableLiteral(_) => todo!(),
        };
    }
}

#[cfg(test)]
mod compiler_tests {
    use crate::compiler::parser2::ir_bytecode_compiler::IRCompiler;
    use crate::compiler::parser2::vm2::bytecode::Bytecode;
    use crate::compiler::parser2::vm2::pointers::ConstantPointer;
    use crate::compiler::parser2::vm2::value::StackValue;
    use crate::compiler::parser2::vm2::vm::Vm;

    #[test]
    pub fn constants() {
        let chunk = IRCompiler::compile_string("1");
        assert_eq!(chunk.bytecode, vec![Bytecode::LoadConstant(ConstantPointer(0))]);
        assert_eq!(chunk.constants, vec![StackValue::Number(1.0)]);
        let chunk = IRCompiler::compile_string("false");
        assert_eq!(chunk.bytecode, vec![Bytecode::LoadConstant(ConstantPointer(0))]);
        assert_eq!(chunk.constants, vec![StackValue::Boolean(false)]);
        let chunk = IRCompiler::compile_string("'h'");
        assert_eq!(chunk.bytecode, vec![Bytecode::LoadConstant(ConstantPointer(0))]);
        assert_eq!(chunk.constants, vec![StackValue::Char('h')]);
    }
    #[test]
    pub fn uninitialized() {
        let chunk = IRCompiler::compile_string("let foo;");
        assert_eq!(chunk.bytecode, vec![Bytecode::LoadConstant(ConstantPointer(0)), Bytecode::PushLocal, Bytecode::Pop])
    }
    #[test]
    pub fn single_assignment() {
        let chunk = IRCompiler::compile_string("let x = 1; x");
        let mut vm = Vm::new();
        vm.load_chunk(chunk);
        vm.run();
        assert_eq!(vm.local, vec![StackValue::Number(1.0)])
    }
    #[test]
    pub fn scoping() {
        let src = r#"
        let x = 1;
        {
            let x = 2;
            x
        }
        "#;
        let mut vm = Vm::new();
        vm.load_chunk(IRCompiler::compile_string(src));
        vm.run();
        assert_eq!(vm.local, vec![StackValue::Number(1.0), StackValue::Number(2.0)])
    }
    #[test]
    pub fn reassignment() {
        let src = r#"
        let x = 1;
        {
            x = 2;
        }
        x
        "#;
        let mut vm = Vm::new();
        vm.load_chunk(IRCompiler::compile_string(src));
        vm.run();
        assert_eq!(vm.local, vec![StackValue::Number(2.0)]);
    }
}