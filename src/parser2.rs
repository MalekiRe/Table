use std::collections::HashMap;
use chumsky::{Parser, select};
use chumsky::prelude::{just, Recursive, Simple};
use indexmap::IndexMap;
use crate::lexer::{BooleanValues, Span, Token};
use crate::parser::FunctionBody;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(BooleanValues),
    Table(IndexMap<TableKeyHolder, Value>),
    Function(FnClosure)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TableKeyHolder {
    String(String),
    IndexOnly(usize)
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "n: {}", number),
            Value::String(string) => write!(f, "s: {}", string),
            Value::Table(table) => write!(f, "t: {:#?}", table),
            Value::Function(function_closure) => write!(f, "f: {:#?}", function_closure),
            Value::Boolean(boolean) => {
                match boolean {
                    BooleanValues::True => write!(f, "b: true"),
                    BooleanValues::False => write!(f, "b: false"),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    And,
    Or,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FnClosure {
    name: String,
}
pub type BExp = Box<Spanned<Exp>>;
#[derive(Debug)]
pub enum Exp {
    Value(Value),
    Binary(BExp, BinaryOp, BExp),
    LocalVar(String),
    StatementsExp(Vec<Statement>, BExp),
    FnCall(FnCall),
    Error,
}
#[derive(Debug)]
pub enum Statement {
    Statements(Vec<Statement>),
    ExpStatement(BExp),
    FnDef(FnDef),
    Let(LetStatement),
}
#[derive(Debug)]
pub enum FnBody {
    StatementsExp {
        statements: Vec<Statement>,
        exp: BExp,
    },
    Exp {
        exp: BExp,
    },
    Statements {
        statements: Vec<Statement>,
    },
    Empty //should be a braced empty, like `{ }`
}
#[derive(Debug)]
pub struct FnDef {
    pub identifier: String,
    pub args: Vec<String>,
    pub fn_body: FnBody,
}
#[derive(Debug)]
pub struct FnCall {
    pub identifier: String,
    pub args: Vec<BExp>,
}
#[derive(Debug)]
pub struct LetStatement {
    pub identifier: String,
    pub value: BExp,
}
pub struct ParserFile(pub FunctionBody);
pub fn file_parser() -> impl Parser<Token, Spanned<Exp>, Error = Simple<Token>> + Clone {
    //let file_parse = Recursive::declare();
    let mut exp = Recursive::declare();
    //let statement = Recursive::declare();
    exp.define({
        let val = select!{
            Token::Number(n) => Exp::Value(Value::Number(n.parse().unwrap())),
            Token::String(string) => Exp::Value(Value::String(string)),
            Token::Boolean(BooleanValues::True) => Exp::Value(Value::Boolean(BooleanValues::True)),
            Token::Boolean(BooleanValues::False) => Exp::Value(Value::Boolean(BooleanValues::False)),
        }.labelled("value");
        let identifier = select! {
            Token::Identifier(string) => Exp::LocalVar(string)
        }.labelled("identifier");
        let items = exp.clone()
            .clone()
            .separated_by(just(Token::Control(',')))
            .allow_trailing();
        let atom = val
            .or(identifier)
            .map_with_span(|expr, span| (expr, span));
        atom

    });
    exp
}