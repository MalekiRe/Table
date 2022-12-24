use std::cell::RefCell;
use std::collections::HashMap;
use std::os::linux::raw::stat;
use std::process::id;
use std::sync::{Arc, Mutex};
use crate::parser::{Atom, Exp, File, FnCall, FunctionBody, LetStatement, Literal, Statement};

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Fn(FnThing),
    None,
}
#[derive(Debug, Clone)]
struct FnThing {
    name: String,
    internal_state: Scope,
}
struct ScopeVal {pub scope: Scope, pub val: Value}
#[derive(Clone, Debug)]
struct Scope {
    inner: HashMap<String, Value>,
    parent: Option<Box<Scope>>
}
impl Scope {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            parent: None
        }
    }
    pub fn push_val(&mut self, identifier: String, value: Value) {
        self.inner.insert(identifier, value);
    }
    pub fn push(self) -> Self {
        Self {
            inner: Default::default(),
            parent: Some(Box::new(self))
        }
    }
    pub fn pop(self) -> Option<Box<Self>> {
        self.parent
    }
    pub fn resolve(&self, name: &str) -> Option<Value> {
        match self.inner.get(name) {
            None => {
                match &self.parent {
                    None => {
                        None
                    }
                    Some(parent) => {
                        parent.resolve(name)
                    }
                }
            }
            Some(val) => {
                Some(val.clone())
            }
        }
    }
}
fn evaluate_exp(scope: Scope, exp: Exp) -> ScopeVal {
    match exp {
        Exp::Atom(atom) => {
            evaluate_atom(scope, atom)
        }
        Exp::FnCall(fn_call) => {
            evaluate_fn_call(scope, fn_call)
        }
        Exp::ExprBraced(boxed_exp) => {
            evaluate_exp(scope, *boxed_exp)
        }
        Exp::StatementExp(statement, exp) => {
            let scope = evaluate_statement(scope, statement);
            evaluate_exp(scope, *exp)
        }
    }
}
fn evaluate_fn_call(scope: Scope, fn_call: FnCall) -> ScopeVal {
    unimplemented!()
}
fn evaluate_statement(mut scope: Scope, statement: Statement) -> Scope {
    match statement {
        Statement::Braced(statements) => {
            for statement in statements {
                scope = evaluate_statement(scope, statement);
            }
            scope
        }
        Statement::Expr(boxed_exp) => {
            evaluate_exp(scope, *boxed_exp).scope
        }
        Statement::FnDef(fn_def) => {
            unimplemented!()
        }
        Statement::LetStatement(let_statement) => {
            evaluate_let_statement(scope, let_statement)
        }
        Statement::EmptyStatement => {
            scope
        }
    }
}
fn evaluate_let_statement(mut scope: Scope, let_statement: LetStatement) -> Scope {
    let scope_val = evaluate_exp(scope, *let_statement.value);
    match scope_val {
        ScopeVal { mut scope, val } => {
            scope.push_val(let_statement.identifier, val);
            scope
        }
    }
}
fn evaluate_atom(mut scope: Scope, atom: Atom) -> ScopeVal {
    match atom {
        Atom::Literal(literal) => evaluate_literal(scope, literal),
        Atom::Variable(variable) => {
            let val = scope.resolve(variable.as_str()).unwrap();
            ScopeVal { scope, val }
        }
    }
}
fn evaluate_literal(scope: Scope, literal: Literal) -> ScopeVal {
    ScopeVal {
        scope,
        val: Value::Literal(literal)
    }
}
fn evaluate_fn_body(scope: Scope, fn_body: FunctionBody) -> ScopeVal {
    match fn_body {
        FunctionBody::Expr(boxed_exp) =>
            evaluate_exp(scope, *boxed_exp),
        FunctionBody::Statement(boxed_statement) => {
            let scope = evaluate_statement(scope, *boxed_statement);
            ScopeVal{ scope, val: Value::None }
        }
    }
}
pub fn evaluate_file(file: File) -> Value {
    let scope = Scope::new();
    match file {
        File::FunctionBody(fn_body) =>
            evaluate_fn_body(scope, fn_body).val,
        File::Empty =>
            Value::None,
    }
}