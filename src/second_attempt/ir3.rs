use chumsky::chain::Chain;
use crate::second_attempt::c_gen_helper::*;
use crate::second_attempt::ir;
use crate::second_attempt::ir::{Block, Exp, File, FnCall, FnDef, LetStatement, Statement, Value};

#[derive(Default)]
pub struct Scope {
    var_declare: Buffer,
    var_in_scope: Vec<CIdentifier>,
    buffer: Buffer,
    var_increment: Buffer,
    var_decrement: Buffer,
    end_buffer: Buffer,
    num_local: u32,
}
impl Scope {
    pub fn buffer(&mut self, buffer: String) {
        self.buffer.push_str(buffer.as_str())
    }
    pub fn var_increment(&mut self, buffer: Buffer) {
        self.var_increment.push_str(buffer.as_str())
    }
    pub fn var_decrement(&mut self, buffer: Buffer) {
        self.var_decrement.push_str(buffer.as_str())
    }
    pub fn var_declare(&mut self, buffer: Buffer, var_identifier: CIdentifier) {
        self.var_declare.push_str(buffer.as_str());
        self.var_in_scope.push(var_identifier);
    }
    pub fn end_buffer(&mut self, buffer: String) {
        self.end_buffer.push_str(buffer.as_str())
    }
    pub fn gen_buffer(self) -> Buffer {
        match self {
            Scope { var_declare, buffer, var_increment, var_decrement, end_buffer, num_local, var_in_scope } => {
                format!("{}{}{}{}{}", var_declare, buffer, var_increment, var_decrement, end_buffer)
            }
        }
    }
}
pub struct ScopeHolder {
    scopes: Vec<Scope>,
    stack: Vec<CIdentifier>,
}
impl ScopeHolder {
    pub fn new() -> Self {
        let mut this = Self {
            scopes: vec![],
            stack: vec![]
        };
        this.push_scope();
        this
    }
    fn get_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
    pub fn get_level(&self) -> u32 {
        (self.scopes.len() - 1 )as u32
    }
    pub fn get_num_local(&mut self) -> &mut u32 {
        &mut self.scopes.last_mut().unwrap().num_local
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }
    pub fn pop_scope(&mut self) {
        let child = self.scopes.pop().unwrap();
        self.scopes.last_mut().unwrap().buffer(child.gen_buffer())
    }
    pub fn push_identifier(&mut self, identifier: CIdentifier) {
        self.stack.push(identifier)
    }
    pub fn pop_identifier(&mut self) -> CIdentifier {
        self.stack.pop().unwrap()
    }
    pub fn var_increment(&mut self, identifier: CIdentifier) {
        self.get_mut().var_increment(generate_increment(identifier))
    }
    pub fn var_decrement(&mut self, identifier: CIdentifier) {
        self.get_mut().var_decrement(generate_decrement(identifier))
    }
    pub fn var_declaration(&mut self, identifier: CIdentifier, rhs: Buffer) {
        self.get_mut().var_declare(generate_variable_declaration(identifier.clone(), rhs), identifier)
    }
    pub fn push_buffer(&mut self, buffer: Buffer) {
        self.get_mut().buffer(buffer);
    }
    pub fn push_end_buffer(&mut self, buffer: Buffer) {
        self.get_mut().end_buffer(buffer);
    }
    pub fn generate_string(mut self) -> Buffer {
        self.scopes.pop().unwrap().gen_buffer()
    }
    pub fn generate_inline_identifier(&mut self) -> CIdentifier {
        generate_inline_identifier(self.get_level(), self.get_num_local())
    }
    pub fn generate_function_identifier(&mut self, identifier: TIdentifier) -> CIdentifier {
        let ident = generate_function_identifier(identifier, self.get_level());
        self.get_mut().var_in_scope.push(ident.clone());
        ident
    }
    pub fn generate_variable_identifier(&mut self, identifier: TIdentifier) -> CIdentifier {
        generate_variable_identifier(identifier, self.get_level())
    }
    pub fn find_var_in_scope(&self, identifier: TIdentifier) -> Option<CIdentifier> {
        for (i, scope) in self.scopes.iter().enumerate() {
            let new_ident = format!("_{}_{}", identifier.clone(), i);
            if scope.var_in_scope.contains(&new_ident) {
                return Some(new_ident);
            }
        }
        None
    }
}

pub type CFnHeader = String;
pub type CFnDef = String;
#[derive(Default)]
pub struct TranslationUnit {
    c_fn_headers: Vec<CFnHeader>,
    c_fn_defs: Vec<CFnDef>,
}
impl TranslationUnit {
    pub fn gen_from_file(file: ir::File) -> Buffer {
        let mut scope = ScopeHolder::new();
        let mut this = Self::default();
        match file {
            File::Block(block) => {
                this.unscoped_block(&mut scope, block)
            }
            File::None => {}
        };
        let fn_body = scope.generate_string();
        this.c_fn_headers.push("Value* _main();".to_string());
        this.c_fn_defs.push(format!("Value* _main(){{{}}}", fn_body));
        this.generate_string()
    }
    fn generate_string(mut self) -> Buffer {
        let mut buffer = Buffer::default();
        for header in &self.c_fn_headers {
            buffer.push_str(header.as_str())
        }
        for def in &self.c_fn_defs {
            buffer.push_str(def.as_str())
        }
        buffer
    }
    /// this one should be used by all but our main function
    fn block(&mut self, scope: &mut ScopeHolder, block: ir::Block) {
        scope.push_scope();
        self.unscoped_block(scope, block);
        scope.pop_scope();
    }
    fn unscoped_block(&mut self, scope: &mut ScopeHolder, block: ir::Block) {
        match block {
            Block::WithExp(statements, exp) => {
                for statement in statements {
                    self.statement(scope, *statement);
                }
                self.expression(scope, *exp);
                let last_var = scope.stack.last().unwrap().clone();
                scope.var_increment(last_var.clone());
                if scope.len() == 1 {
                    scope.push_end_buffer(generate_return_line(last_var));
                }
            }
            Block::WithoutExp(statements) => {
                for statement in statements {
                    self.statement(scope, *statement);
                }
                if scope.len() == 1 {
                    scope.push_end_buffer("return None();".to_string());
                }
            }
        }
    }
    fn expression(&mut self, scope: &mut ScopeHolder, exp: ir::Exp) {
        match exp {
            Exp::FnCall(fn_call) => self.fn_call(scope, fn_call),
            Exp::BinaryOperation(_) => unimplemented!(),
            Exp::Value(value) => self.value(scope, value),
            Exp::Variable(_) => unimplemented!(),
            Exp::Block(_, _) => unimplemented!(),
        }
    }
    fn fn_call(&mut self, scope: &mut ScopeHolder, fn_call: ir::FnCall) {
        match fn_call {
            FnCall { identifier, args } => {
                let arg_len = args.len();
                for arg in args {
                    self.expression(scope, *arg);
                }
                let inline_ret = scope.generate_inline_identifier();
                let closure_name = scope.find_var_in_scope(identifier).unwrap();
                let mut arg_idents = vec![];
                for i in 0..arg_len {
                    arg_idents.push(scope.pop_identifier());
                }
                let buffer = format!("(*{}->variant.closure->p)({})", closure_name, call_args_to_string(arg_idents));
                scope.var_declaration(inline_ret.clone(), buffer);
                scope.push_identifier(inline_ret);
            }
        }
    }
    fn value(&mut self, scope: &mut ScopeHolder, value: ir::Value) {
        let inline_name = scope.generate_inline_identifier();
        scope.var_declaration(inline_name.clone(), generate_value_new(value));
        scope.push_identifier(inline_name);
    }
    fn statement(&mut self, scope: &mut ScopeHolder, statement: ir::Statement) {
        match statement {
            Statement::FnDef(fn_def) => self.fn_def(scope, fn_def),
            Statement::LetStatement(let_statement) => self.let_statement(scope, let_statement),
            Statement::ExpStatement(exp_statement) => {
                self.expression(scope, *exp_statement);
                scope.stack.pop().unwrap();
            },
            Statement::Block(_) => unimplemented!(),
        }
    }
    fn let_statement(&mut self, scope: &mut ScopeHolder, let_stmt: LetStatement) {
        match let_stmt {
            LetStatement { identifier, exp } => {
                self.expression(scope, *exp);
                let c_identifier = scope.generate_variable_identifier(identifier);
                let exp_ident = scope.stack.pop().unwrap();
                scope.var_declaration(c_identifier.clone(), exp_ident);
            }
        }
    }
    fn fn_def(&mut self, scope: &mut ScopeHolder, fn_definition: FnDef) {
        match fn_definition {
            FnDef { identifier, args, body, closure_idents } => {
                let args = args_to_string(args);
                let fn_identifier = scope.generate_function_identifier(identifier);
                let fn_header = generate_function_header(fn_identifier.clone(), args.clone());
                let mut fn_scope = ScopeHolder::new();
                let mut new_closure_idents = vec![];
                for closure_ident in &closure_idents {
                    new_closure_idents.push(scope.find_var_in_scope(closure_ident.clone()).unwrap());
                }
                let inline_dec = scope.generate_inline_identifier();
                let closure_generation = generate_closure_declaration(inline_dec, fn_identifier.clone(), new_closure_idents);
                for (i, closure_ident) in closure_idents.iter().enumerate() {
                    let id = fn_scope.generate_variable_identifier(closure_ident.clone());
                    fn_scope.var_declaration(id, format!("args[{}]", i));
                }
                self.unscoped_block(&mut fn_scope, body);
                let fn_body = fn_scope.generate_string();
                let fn_def = generate_function_def(fn_identifier.clone(), args, fn_body);
                scope.get_mut().var_declare.push_str(closure_generation.as_str());
                self.c_fn_defs.push(fn_def);
                self.c_fn_headers.push(fn_header);
            }
        };
    }
}