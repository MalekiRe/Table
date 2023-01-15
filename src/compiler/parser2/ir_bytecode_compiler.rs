use std::collections::HashMap;
use crate::compiler::ir::{IdentifierT};
use crate::compiler::parser2::parser::parse;
use crate::compiler::parser2::parsing_ir::{Block, BStatement, Exp, ExpBlock, File, FnCall, FnCallArgs, FnDec, FnDecArgs, LetStatement, Literal, ReassignStatement, Statement, StatementBlock, Variable};
use crate::compiler::parser2::vm2::bytecode::Bytecode;
use crate::compiler::parser2::vm2::chunk::Chunk;
use crate::compiler::parser2::vm2::pointers::{ChunkPointer, ConstantPointer, HeapPointer, LocalDistance};
use crate::compiler::parser2::vm2::value::StackValue;


#[derive(Debug, Copy, Clone)]
pub enum Location {
    Heap(HeapPointer),
    Local(LocalDistance),
}

pub struct Scope {
    anonymous_locals: usize,
    local_variables: HashMap<IdentifierT, usize>,
    heap_variables: HashMap<IdentifierT, HeapPointer>,
}

pub struct ScopeHolder {
    heap_size: usize,
    scopes: Vec<Scope>
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            anonymous_locals: 0,
            local_variables: Default::default(),
            heap_variables: Default::default(),
        }
    }
}

impl Default for ScopeHolder {
    fn default() -> Self {
        Self {
            heap_size: 0,
            scopes: vec![Scope::default()]
        }
    }
}

impl ScopeHolder {
    pub fn push_local_var(&mut self, identifier: IdentifierT) {
        let scope = self.scopes.last_mut().unwrap();
        let len = scope.local_variables.len();
        let anon_locals = scope.anonymous_locals;
        scope.local_variables.insert(identifier, len);
    }
    pub fn push_anon_local(&mut self) {
        self.scopes.last_mut().unwrap().anonymous_locals += 1;
    }
    pub fn find_var(&self, identifier: IdentifierT) -> Option<Location> {
        let mut scope_index = self.scopes.len();
        loop {
            if scope_index == 0 { break; }
            scope_index -= 1;
            match self.scopes[scope_index].local_variables.get(identifier.as_str()) {
                None => {}
                Some(location) => {
                    let len = self.scopes[scope_index].local_variables.len() - 1;
                    let anon_local = self.scopes[scope_index].anonymous_locals;
                    let distance = LocalDistance((len + anon_local) - *location);
                    return Some(Location::Local(distance))
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
    chunk: Vec<Chunk>,
    scope_holder: ScopeHolder,
    local_count: usize,
}
impl IRCompiler {
    fn new() -> Self {
        Self {
            chunk: vec![Chunk::default()],
            scope_holder: Default::default(),
            local_count: 0,
        }
    }
    pub fn compile_string(code: &str) -> Chunk {
        let mut this = Self::new();
        this.compile_file(parse(code).unwrap())
    }
    pub fn compile_file(mut self, file: File) -> Chunk {
        self.file(file);
        match self {
            IRCompiler { mut chunk, .. } => {
                chunk.pop().unwrap()
            }
        }
    }
    pub fn push_code(&mut self, bytecode: Bytecode) {
        match bytecode {
            Bytecode::PushLocal => self.local_count += 1,
            Bytecode::PopLocal => self.local_count -= 1,
            _ => {}
        }
        self.chunk.last_mut().unwrap().bytecode.push(bytecode);
    }
    pub fn add_local_var(&mut self, identifier: IdentifierT) {
        self.scope_holder.push_local_var(identifier)
    }
    pub fn add_constant(&mut self, stack_value: StackValue) -> ConstantPointer {
        //TODO add caching so we don't add constants we already have
        let ptr = ConstantPointer(self.chunk.last().unwrap().constants.len());
        self.push_code(Bytecode::LoadConstant(ptr));
        self.chunk.last_mut().unwrap().constants.push(stack_value);
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
                        self.push_code(Bytecode::SetLocal(local_pointer));
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
                self.push_code(Bytecode::PushLocal);
                self.push_code(Bytecode::Pop);
            }
            LetStatement::SingleAssign(identifier, b_exp) => {
                self.add_local_var(identifier);
                self.exp(*b_exp);
                self.push_code(Bytecode::PushLocal);
                self.push_code(Bytecode::Pop);
            },
            LetStatement::Table(_) => todo!(),
            LetStatement::UniqueIdentTable(_) => todo!(),
        }
    }

    pub fn exp(&mut self, exp: Exp) {
        match exp {
            Exp::BinaryExp(_) => todo!(),
            Exp::ExpBlock(exp_block) => self.exp_block(exp_block),
            Exp::FnCall(fn_call) => self.fn_call(fn_call),
            Exp::FnDec(fn_dec) => self.fn_dec(fn_dec),
            Exp::TableExp(_) => todo!(),
            Exp::ControlFlowExp(_) => todo!(),
            Exp::RangeCreation(_) => todo!(),
            Exp::MacroCall(_) => todo!(),
            Exp::LiteralCode(_) => todo!(),
            Exp::Literal(literal) => self.literal(literal),
            Exp::VariableIdentifier(variable_identifier) => self.variable_identifier(variable_identifier),
        }
    }
    pub fn fn_call(&mut self, fn_call: FnCall) {
        match fn_call {
            FnCall { ident, fn_call_args } => {
                let mut len = None;
                match fn_call_args {
                    FnCallArgs { args } => {
                        len = Some(args.len());
                        for (i, b_exp) in args.into_iter().enumerate() {
                            self.exp(*b_exp);
                            self.push_code(Bytecode::PushLocal);
                            self.scope_holder.push_anon_local();
                            self.push_code(Bytecode::Pop);
                        }
                    }
                }
                let location = self.scope_holder.find_var(ident).unwrap();
                match location {
                    Location::Heap(_) => todo!(),
                    Location::Local(local_distance) => {
                        self.push_code(Bytecode::PeekLocal(local_distance))
                    }
                }
                self.push_code(Bytecode::RunChunk);
            }
        }
    }
    pub fn block(&mut self, block: Block) {
        match block {
            Block::Exp(b_exp) => self.exp(*b_exp),
            Block::Statement(b_statement) => self.statement(*b_statement),
        }
    }
    pub fn fn_dec(&mut self, fn_dec: FnDec) {
        self.chunk.push(Chunk::default());
        self.scope_holder.push_scope();
        match fn_dec {
            FnDec { dec_args, body } => {
                match &dec_args {
                    FnDecArgs { args } => {
                        for  arg in args {
                            self.scope_holder.push_local_var(arg.clone());
                        }
                    }
                }
                self.block(body);
                self.push_code(Bytecode::Return);
            }
        }
        self.scope_holder.pop_scope();
        let chunk = self.chunk.pop().unwrap();
        self.chunk.last_mut().unwrap().chunks.push(chunk);
        let chunk_pointer = ChunkPointer(self.chunk.last_mut().unwrap().chunks.len()-1);
        self.add_constant(StackValue::Chunk(chunk_pointer));
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
            Location::Local(local_distance) => {
                self.push_code(Bytecode::PeekLocal(local_distance))
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
    #[test]
    pub fn fn_dec() {
        let src = r#"
        let foo = (a) -> a;
        foo(1)
        "#;
        let mut vm = Vm::new();
        vm.load_chunk(IRCompiler::compile_string(src));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(1.0)])
    }
    #[test]
    pub fn more_complex_fn() {
        let src = r#"
        let foo = (a) -> a;
        let bar = (a, b) -> b;
        foo(bar(1, 2))
        "#;
        let mut vm = Vm::new();
        //panic!("{:#?}", IRCompiler::compile_string(src));
        vm.load_chunk(IRCompiler::compile_string(src));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(2.0)])
    }
}