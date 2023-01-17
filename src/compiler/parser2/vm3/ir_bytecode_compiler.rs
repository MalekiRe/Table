use std::collections::HashMap;
use chumsky::chain::Chain;
use crate::compiler::parser2::ir_bytecode_compiler::Location;
use crate::compiler::parser2::parser::parse;
use crate::compiler::parser2::parsing_ir::{Block, BStatement, Exp, ExpBlock, File, FnCall, FnDec, IdentifierT, LetStatement, Literal, Statement};
use crate::compiler::parser2::vm3::bytecode::Bytecode;
use crate::compiler::parser2::vm3::chunk::Chunk;
use crate::compiler::parser2::vm3::pointer::{ChunkPointer, ConstantPointer, HeapPointer, LocalDistance};
use crate::compiler::parser2::vm3::value::StackValue;

pub struct Scope {
    scope_type: ScopeType,
    anon_locals: usize,
    variables: HashMap<IdentifierT, Variable>,
}
#[derive(Clone)]
pub enum Variable {
    Heap(LocalDistance),
    Stack(LocalDistance),
}
#[derive(PartialEq, Clone, Copy)]
pub enum ScopeType {
    Standard,
    Closure,
}
impl Scope {
    pub fn new(scope_type: ScopeType) -> Self {
        Self {
            scope_type,
            anon_locals: 0,
            variables: Default::default()
        }
    }
}

pub struct ScopeHolder {
    scopes: Vec<Scope>,
}
impl Default for ScopeHolder {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::new(ScopeType::Standard)]
        }
    }
}
impl ScopeHolder {
    pub fn push_standard_scope(&mut self) {
        self.scopes.push(Scope::new(ScopeType::Standard))
    }
    pub fn push_closure_scope(&mut self) {
        self.scopes.push(Scope::new(ScopeType::Closure))
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop().unwrap();
    }
    pub fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
    pub fn push_anon_local(&mut self) {
        self.scope_mut().anon_locals += 1;
    }
    pub fn pop_anon_local(&mut self) {
        self.scope_mut().anon_locals -= 1;
    }
    pub fn push_local_variable(&mut self, identifier: IdentifierT) {
        let len = self.scope_mut().variables.len();
        self.scope_mut().variables.insert(identifier, Variable::Stack(LocalDistance(len)));
    }
    pub fn push_heap_variable(&mut self, identifier: IdentifierT) {
        let len = self.scope_mut().len();
        self.scope_mut().variables.insert(identifier, Variable::Heap(LocalDistance(len)));
    }
    pub fn find_variable(&mut self, identifier: IdentifierT) -> Option<(Variable, ScopeType)> {
        let mut scope_index = self.scopes.len();
        let mut scope_type = ScopeType::Standard;
        loop {
            if scope_index == 0 { break; }
            scope_index -= 1;
            match self.scopes[scope_index].variables.get(identifier.as_str()) {
                None => {}
                Some(variable) => {
                    let anon_local = self.scopes[scope_index].anon_locals;
                    let len = self.scopes[scope_index].variables.len() - 1;
                    return Some((match variable {
                        Variable::Heap(distance) => {
                            let distance = (len + anon_local) - distance.0;
                            Variable::Heap(LocalDistance(distance))
                        }
                        Variable::Stack(distance) => {
                            print!("ident: {}, index: {}", identifier, distance.0);
                            let distance = (len + anon_local) - distance.0;
                            println!(", distance: {}", distance);
                            Variable::Stack(LocalDistance(distance))
                        }
                    }, scope_type));
                }
            }
            match self.scopes[scope_index].scope_type {
                ScopeType::Standard => {}
                ScopeType::Closure => scope_type = ScopeType::Closure,
            }
        }
        None
    }
    pub fn uplift_variable(&mut self, identifier: IdentifierT) {
        let mut scope_index = self.scopes.len();
        loop {
            if scope_index == 0 { break; }
            scope_index -= 1;
            let thing = match self.scopes[scope_index].variables.get(identifier.as_str()) {
                None => None,
                Some(variable) => {
                    Some(Variable::Heap(LocalDistance(match variable {
                        Variable::Heap(_) => panic!(),
                        Variable::Stack(distance) => distance.0
                    })))
                }
            };
            self.scopes[scope_index].variables.remove(identifier.as_str());
            match thing {
                None => {}
                Some(thing) => {
                    self.scopes[scope_index].variables.insert(identifier.clone(), thing);
                }
            }
            return;

        }
    }
}

pub struct IRCompiler {
    chunks: Vec<Chunk>,
    scope_holder: ScopeHolder,
}

impl IRCompiler {
    pub fn prev_chunk(&mut self) -> &mut Chunk {
        let index = self.chunks.len()-2;
        self.chunks.get_mut(index).unwrap()
    }
    pub fn do_closure_scope(&mut self, mut cb: impl Fn(&mut IRCompiler) -> ()) {
        self.scope_holder.push_closure_scope();
        cb(self);
        self.scope_holder.pop_scope();
    }
    pub fn do_standard_scope(&mut self, mut cb: impl Fn(&mut IRCompiler) -> ()) {
        self.scope_holder.push_standard_scope();
        cb(self);
        self.scope_holder.pop_scope();
    }
    pub fn do_chunk(&mut self, mut cb: impl Fn(&mut IRCompiler) -> ()) {
        self.chunks.push(Chunk::default());
        cb(self);
        let chunk = self.chunks.pop().unwrap();
        self.chunks.last_mut().unwrap().chunks.push(chunk);
        let chunk_pointer = ChunkPointer(self.chunks.last_mut().unwrap().chunks.len()-1);
        self.add_constant(StackValue::HeapPointer(HeapPointer::Chunk(chunk_pointer)));
    }
    pub fn push_code(&mut self, bytecode: Bytecode) {
        // match bytecode {
        //     Bytecode::PushLocal => self.local_count += 1,
        //     Bytecode::PopLocal => self.local_count -= 1,
        //     _ => {}
        // }
        self.chunks.last_mut().unwrap().bytecode.push(bytecode);
    }
    pub fn add_constant(&mut self, stack_value: StackValue) -> ConstantPointer {
        //TODO add caching so we don't add constants we already have
        let ptr = ConstantPointer(self.chunks.last().unwrap().constants.len());
        self.push_code(Bytecode::LoadConstant(ptr));
        self.chunks.last_mut().unwrap().constants.push(stack_value);
        ptr
    }

    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::default()],
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
            IRCompiler { mut chunks, .. } => {
                chunks.pop().unwrap()
            }
        }
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
    pub fn block(&mut self, block: Block) {
        match block {
            Block::Exp(b_exp) => self.exp(*b_exp),
            Block::Statement(b_statement) => self.statement(*b_statement),
        }
    }
    pub fn b_statements(&mut self, b_statements: Vec<BStatement>) {
        for b_statement in b_statements {
            self.statement(*b_statement);
        }
    }
    pub fn statement(&mut self, statement: Statement) {
        match statement {
            Statement::StatementBlock(_) => todo!(),
            Statement::LetStatement(let_statement) => self.let_statement(let_statement),
            Statement::ReassignStatement(_) => todo!(),
            Statement::BreakStatement(_) => todo!(),
            Statement::IfStatement(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
            Statement::ExpStatement(_) => todo!(),
            Statement::MacroCall(_) => todo!(),
        }
    }

    pub fn let_statement(&mut self, let_statement: LetStatement) {
        match let_statement {
            LetStatement::Uninitialized(identifier) => {
                self.scope_holder.push_local_variable(identifier);
                self.add_constant(StackValue::Nil);
                self.push_code(Bytecode::PushLocal);
            }
            LetStatement::SingleAssign(identifier, b_exp) => {
                self.scope_holder.push_local_variable(identifier);
                self.add_constant(StackValue::Nil);
                self.push_code(Bytecode::PushLocal);
                self.exp(*b_exp);
                self.push_code(Bytecode::PopLocal);
                self.push_code(Bytecode::PushLocal);
            }
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
    pub fn exp_block(&mut self, exp_block: ExpBlock) {
        match exp_block {
            ExpBlock { statements, exp } => {
                self.b_statements(statements);
                self.exp(*exp);
            }
        }

    }
    pub fn fn_call(&mut self, fn_call: FnCall) {
        match fn_call {
            FnCall { ident, fn_call_args } => {
                for arg in fn_call_args.args {
                    self.exp(*arg);
                    self.push_code(Bytecode::PushLocal);
                    self.scope_holder.push_anon_local();
                }
                self.variable_identifier(ident);
                self.push_code(Bytecode::LoadChunk);
            }
        }
    }
    pub fn fn_dec(&mut self, fn_dec: FnDec) {
        match fn_dec {
            FnDec { dec_args, body } => {
                self.do_closure_scope(|this| {
                    this.do_chunk(|this| {
                        for arg in &dec_args.args {
                            this.scope_holder.push_local_variable(arg.clone());
                        }
                        this.block(body.clone());
                    });
                });
            }
        }
    }
    pub fn literal(&mut self, literal: Literal) {
        match literal {
            Literal::Char(char) =>
                self.add_constant(StackValue::Char(char)),
            Literal::Number(number) =>
                self.add_constant(StackValue::Number(number)),
            Literal::Boolean(bool) =>
                self.add_constant(StackValue::Boolean(bool)),
            Literal::String(_) => todo!(),
            Literal::TableLiteral(_) => todo!(),
        };
    }
    pub fn variable_identifier(&mut self, variable_identifier: IdentifierT) {
        let (location, scope_type) = self.scope_holder.find_variable(variable_identifier.clone()).unwrap();
        match location {
            Variable::Heap(_) => todo!(),
            Variable::Stack(local_distance) => {
                match scope_type {
                    ScopeType::Standard => {
                        self.push_code(Bytecode::PeekLocal(local_distance))
                    }
                    ScopeType::Closure => {
                        self.prev_chunk().bytecode.push(Bytecode::UpValueLocal(local_distance.clone()));
                        self.push_code(Bytecode::PeekLocal(local_distance));
                        self.scope_holder.uplift_variable(variable_identifier);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod fullstack_tests {
    use crate::compiler::parser2::vm3::ir_bytecode_compiler::IRCompiler;
    use crate::compiler::parser2::vm3::pointer::{ChunkPointer, HeapPointer};
    use crate::compiler::parser2::vm3::value::StackValue;
    use crate::compiler::parser2::vm3::vm::Vm;

    #[test]
    fn empty() {
        let src = "";
        let chunk = IRCompiler::compile_string(src);
        let mut vm = Vm::new(chunk);
        vm.run();
    }

    #[test]
    fn literal_exp() {
        let mut vm = Vm::new(IRCompiler::compile_string("1"));
        vm.run();
        assert_eq!(vm.chunk_ref().stack, vec![StackValue::Number(1.0)]);
    }

    #[test]
    fn let_statement() {
        let mut vm = Vm::new(IRCompiler::compile_string("let x = 1;"));
        vm.run();
        assert_eq!(vm.local, vec![StackValue::Number(1.0)]);
        let chunk = IRCompiler::compile_string("let x = 1; let y = 2; x");
        let mut vm = Vm::new(chunk);
        vm.run();
        assert_eq!(vm.chunk_ref().stack, vec![StackValue::Number(1.0)]);
    }
    #[test]
    fn fn_dec() {
        let mut vm = Vm::new(IRCompiler::compile_string("let foo = (a) -> a;"));
        vm.run();
        assert_eq!(vm.local, vec![StackValue::HeapPointer(HeapPointer::Chunk(ChunkPointer(0)))]);
    }
    #[test]
    fn fn_call() {
        let src = r#"
        let foo = (a) -> a;
        foo(1)
        "#;
        let mut vm = Vm::compile_str(src);
        vm.run();
        assert_eq!(vm.chunk_ref().stack, vec![StackValue::Number(1.0)]);
    }
    #[test]
    fn closure() {
        let src = r#"
        let x = 2;
        let foo = () -> x;
        foo()
        "#;
        let chunk = IRCompiler::compile_string(src);
        println!("{:#?}", chunk);
        let mut vm = Vm::compile_str(src);
        vm.run();
        assert_eq!(vm.chunk_ref().stack, vec![StackValue::Number(2.0)])
    }
    #[test]
    fn multiple_def() {
        let src = r#"
        let z = 2;
        let x = { let y = 1; y};
        x
        "#;
        let chunk = IRCompiler::compile_string(src);
        println!("{:#?}", chunk);
    }
}