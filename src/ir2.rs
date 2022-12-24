use std::collections::HashMap;
use indexmap::IndexMap;
use crate::parser2::{Exp, FnBody, LetStatement, ParserFile, PrimitiveValue, Statement, TableKey};

#[derive(Clone, Debug)]
pub enum Value {
    PrimitiveValue(PrimitiveValue),
    TableValue(TableValue),
    None,
}
#[derive(Clone, Debug)]
struct TableValue(pub IndexMap<TableKey, Value>);

struct ScopeVal {pub scope: Scope, pub val: Value}
impl ScopeVal {
    pub fn from(scope: Scope, val: Value) -> Self {
        ScopeVal{
            scope,
            val
        }
    }
}
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
pub fn evaluate_file(parser_file: ParserFile) -> Value {
    match parser_file {
        ParserFile::StatementsExp(statements, bexp) => {
            let mut scope = Scope::new();
            for statement in statements {
                scope = evaluate_statement(scope, statement);
            }
            evaluate_exp(scope, *bexp).val
        }
        ParserFile::Statements(statements) => {
            unimplemented!()
        }
    }
}
fn evaluate_fn_body(mut scope: Scope, fn_body: FnBody) -> ScopeVal {
    match fn_body {
        FnBody::StatementsExp { .. } => {unimplemented!()}
        FnBody::Statements { .. } => {unimplemented!()}
        FnBody::Statement(_) => {unimplemented!()}
        FnBody::Exp(bexp) => {
            evaluate_exp(scope, *bexp)
        }
        FnBody::Empty => {
            ScopeVal::from(scope, Value::None)
        }
    }
}
fn evaluate_exp(mut scope: Scope, exp: Exp) -> ScopeVal {
    match exp {
        Exp::PrimitiveValue(primitive_value) => {
            ScopeVal::from(scope, Value::PrimitiveValue(primitive_value))
        }
        Exp::Table(_) => {unimplemented!()}
        Exp::Binary(_, _, _) => {unimplemented!()}
        Exp::LocalVar(local_var) => {
            let val = scope.resolve(local_var.as_str()).unwrap();
            ScopeVal::from(scope, val)
        }
        Exp::StatementsExp(statements, bexp) => {
            let mut scope = Scope::new();
            for statement in statements {
                scope = evaluate_statement(scope, statement);
            }
            evaluate_exp(scope, *bexp)
        }
        Exp::FnCall(_) => {unimplemented!()}
        Exp::Error => {unreachable!()}
    }
}
fn evaluate_statement(mut scope: Scope, statement: Statement) -> Scope {
    match statement {
        Statement::FnDef(_) => {unimplemented!()}
        Statement::Statements(_) => {unimplemented!()}
        Statement::ExpStatement(_) => {unimplemented!()}
        Statement::Let(let_statement) => {
            evaluate_let_statement(scope, let_statement)
        }
    }
}
fn evaluate_let_statement(mut scope: Scope, let_statement: LetStatement) -> Scope {
    match evaluate_exp(scope,*let_statement.value) {
        ScopeVal { mut scope, val } => {
            scope.push_val(let_statement.identifier, val);
            scope
        }
    }

}