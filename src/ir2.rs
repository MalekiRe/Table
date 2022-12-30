use std::collections::HashMap;
use indexmap::IndexMap;
use crate::parser2::{BinaryOp, Exp, FnBody, FnCall, FnDef, LetStatement, ParserFile, PrimitiveValue, Statement, TableKey};

#[derive(Clone, Debug)]
pub enum Value {
    PrimitiveValue(PrimitiveValue),
    TableValue(TableValue),
    FnDef(FnClosure),
    None,
}

#[derive(Clone, Debug)]
pub struct FnClosure {
    args: Vec<String>,
    scope: Scope,
    fn_body: FnBody,
}

#[derive(Clone, Debug)]
pub struct TableValue(pub IndexMap<TableKey, Value>);

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
    parent: Option<Box<Scope>>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            parent: None,
        }
    }
    pub fn push_val(&mut self, identifier: String, value: Value) {
        self.inner.insert(identifier, value);
    }
    pub fn push(self) -> Self {
        Self {
            inner: Default::default(),
            parent: Some(Box::new(self)),
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
        FnBody::StatementsExp { statements, exp } => {
            for statement in statements {
                scope = evaluate_statement(scope, statement);
            }
            evaluate_exp(scope, *exp)
        }
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
        Exp::Binary(bexp1, binary_op, bexp2) => {
            evaluate_binary_op(scope, *bexp1, *bexp2, binary_op)
        }
        Exp::LocalVar(local_var) => {
            let val = match scope.resolve(local_var.as_str()) {
                None => panic!("variable: {} does not exist or is not in scope: {:#?}", local_var, scope),
                Some(val) => val,
            };
            ScopeVal::from(scope, val)
        }
        Exp::StatementsExp(statements, bexp) => {
            let mut scope = Scope::new();
            for statement in statements {
                scope = evaluate_statement(scope, statement);
            }
            evaluate_exp(scope, *bexp)
        }
        Exp::FnCall(fn_call) => {
            evaluate_fn_call(scope, fn_call)
        }
        Exp::Error => {unreachable!()}
    }
}
fn evaluate_fn_call(mut scope1: Scope, fn_call: FnCall) -> ScopeVal {
    match fn_call {
        FnCall { identifier, args } => {
            let fn_closure = scope1.resolve(identifier.as_str()).unwrap();
            let mut new_args = Vec::<Value>::new();
            for arg in args {
                let scope_val = evaluate_exp(scope1, *arg);
                let val = scope_val.val;
                scope1 = scope_val.scope;
                new_args.push(val);
            }
            match fn_closure {
                Value::FnDef(fn_closure) => {
                    return match fn_closure {
                        FnClosure { args, scope, fn_body } => {
                            let mut scope = scope.push();
                            for (i, arg) in args.into_iter().enumerate() {
                                scope.push_val(arg, new_args.get(i).unwrap().clone());
                            }
                            println!("{:#?}", scope);
                            let scope_val = evaluate_fn_body(scope, fn_body);
                            ScopeVal::from(scope1, scope_val.val)
                        }
                    }
                }
                _ => panic!("not a fn identifier")
            }
        }
    }
    unimplemented!()
}
fn evaluate_binary_op(mut scope: Scope, exp1: Exp, exp2: Exp, binary_op: BinaryOp) -> ScopeVal {
    let scope_val = evaluate_exp(scope, exp1);
    let val1 = scope_val.val;
    let scope_val = evaluate_exp(scope_val.scope, exp2);
    let val2 = scope_val.val;
    match binary_op {
        BinaryOp::Add => {
            match val1 {
                Value::PrimitiveValue(primitive_val) => {
                    match primitive_val {
                        PrimitiveValue::Number(number1) => {
                            match val2 {
                                Value::PrimitiveValue(primtive_value) => {
                                    match primtive_value {
                                        PrimitiveValue::Number(number2) => {
                                            ScopeVal::from(scope_val.scope, Value::PrimitiveValue(PrimitiveValue::Number(number1+number2)))
                                        }
                                        _ => panic!("val2 of thing is not a number")
                                    }
                                }
                                _ => panic!("val2 of thing is not a primitive value")
                            }
                        }
                        _ => panic!("val1 of thing is not a number")
                    }
                }
                _ => panic!("val1 of thing is not a primitive value")
            }
        }
        BinaryOp::Sub => unimplemented!(),
        BinaryOp::Mul => unimplemented!(),
        BinaryOp::Div => unimplemented!(),
        BinaryOp::Eq => unimplemented!(),
        BinaryOp::NotEq => unimplemented!(),
        BinaryOp::And => unimplemented!(),
        BinaryOp::Or => unimplemented!(),
    }
}
fn evaluate_statement(mut scope: Scope, statement: Statement) -> Scope {
    match statement {
        Statement::FnDef(bfn_def) => {
            evaluate_fn_def(scope, *bfn_def)
        }
        Statement::Statements(_) => {unimplemented!()}
        Statement::ExpStatement(_) => {unimplemented!()}
        Statement::Let(let_statement) => {
            evaluate_let_statement(scope, let_statement)
        }
    }
}
fn evaluate_fn_def(mut scope: Scope, fn_def: FnDef) -> Scope {
    match fn_def {
        FnDef { identifier, args, fn_body, exported } => {
            let fn_closure = FnClosure {
                args,
                scope: scope.clone(),
                fn_body
            };
            scope.push_val(identifier, Value::FnDef(fn_closure))
        }
    }
    scope
}
fn evaluate_let_statement(mut scope: Scope, let_statement: LetStatement) -> Scope {
    match evaluate_exp(scope,*let_statement.value) {
        ScopeVal { mut scope, val } => {
            scope.push_val(let_statement.identifier, val);
            scope
        }
    }

}