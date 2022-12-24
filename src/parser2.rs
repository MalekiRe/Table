use std::collections::HashMap;
use chumsky::{Error, Parser, recovery, select};
use chumsky::prelude::{end, filter_map, just, none_of, Recursive, recursive, Simple, skip_parser, skip_then_retry_until};
use indexmap::IndexMap;
use crate::lexer::{BooleanValues, Span, Token};
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveValue {
    Number(f64),
    String(String),
    Boolean(BooleanValues),
    Function(FnClosure)
}
#[derive(Debug)]
pub struct Table(pub IndexMap<TableKey, Exp>);
impl Table {
    pub fn new() -> Self {
        Table(IndexMap::new())
    }
}
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum TableKey {
    Identifier(String, usize),
    NoIdentifier(usize),
}
impl std::fmt::Display for PrimitiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PrimitiveValue::Number(number) => write!(f, "n: {}", number),
            PrimitiveValue::String(string) => write!(f, "s: {}", string),
            // Value::Table(table) => write!(f, "t: {:#?}", table),
            PrimitiveValue::Function(function_closure) => write!(f, "f: {:#?}", function_closure),
            PrimitiveValue::Boolean(boolean) => {
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
pub type BExp = Box<Exp>;
pub type PExp = Spanned<Exp>;
#[derive(Debug)]
pub enum Exp {
    PrimitiveValue(PrimitiveValue),
    Table(Box<Table>),
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
    FnDef(Box<FnDef>),
    Let(LetStatement),
}
// #[derive(Debug)]
// pub enum FnBody {
//     StatementsExp {
//         statements: Vec<Statement>,
//         exp: BExp,
//     },
//     Exp {
//         exp: BExp,
//     },
//     Statements {
//         statements: Vec<Statement>,
//     },
//     Empty //should be a braced empty, like `{ }`
// }

#[derive(Debug)]
pub enum FnBody {
    Statement(Statement),
    Exp(BExp),
    Empty,
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
#[derive(Debug)]
pub struct ParserFile(pub FnBody);
pub fn file_parser() -> impl Parser<Token, Spanned<ParserFile>, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let mut fn_body = Recursive::declare();
    let mut exp = Recursive::declare();
    let mut statement = Recursive::declare();
    let file_parse =
        fn_body.clone().map(|fn_body| {
            ParserFile(fn_body)
        });
    fn_body.define({
        statement.clone().map(|statement| {
            FnBody::Statement(statement)
        }).or(exp.clone().map(|exp| {
            FnBody::Exp(Box::new(exp))
        }))
            .or(just(Token::Control('{')).then_ignore(just(Token::Control('}'))).map(|_| {
                FnBody::Empty
            }))
    });
    let fn_def = {
        let fn_def_args = ident.clone().separated_by(just(Token::Control(','))).allow_trailing();
        just(Token::Fn)
            .ignore_then(ident.clone())
            .then(fn_def_args.delimited_by(just(Token::Control('(')), just(Token::Control(')'))))
            .then(fn_body)
            .map(|((identifier, fn_args), fn_body)| {
                FnDef {
                    identifier,
                    args: fn_args,
                    fn_body
                }
            })
    };
    statement.define({
        fn_def.map(|fn_def| {
            Statement::FnDef(Box::new(fn_def))
        })
    });
    let mut table_construction = Recursive::declare();
    let fn_call = recursive(|fn_call| {
        let ident = ident.clone();
        let fn_call_args = exp.clone().separated_by(just(Token::Control(','))).allow_trailing()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')')));
        let fn_call = ident.then(fn_call_args).map(|(identifier, args)| {
            println!("{:#?}", args);
            Exp::FnCall(FnCall {
                identifier,
                args: args.into_iter().map(|arg| {Box::new(arg)}).collect()
            })
        });
        fn_call
    });
    exp.define({
        let val = select!{
            Token::Number(n) => Exp::PrimitiveValue(PrimitiveValue::Number(n.parse().unwrap())),
            Token::String(string) => Exp::PrimitiveValue(PrimitiveValue::String(string)),
            Token::Boolean(BooleanValues::True) => Exp::PrimitiveValue(PrimitiveValue::Boolean(BooleanValues::True)),
            Token::Boolean(BooleanValues::False) => Exp::PrimitiveValue(PrimitiveValue::Boolean(BooleanValues::False)),
        }.labelled("value");
        let identifier = select! {
            Token::Identifier(string) => Exp::LocalVar(string)
        }.labelled("identifier");
        let items = exp.clone()
            .separated_by(just(Token::Control(',')))
            .allow_trailing();
        let atom = val
            .or(table_construction.clone().map(|table| {
                Exp::Table(Box::new(table))
            }))
            .or(fn_call)
            .or(identifier);
        atom
    });
    table_construction.define({
        enum TableArgTypes {
            WithIdentifier(String, Exp),
            NoIdentifier(Exp),
        }
        let ident = ident.clone();
        let with_identifier = ident.then_ignore(just(Token::Control(':'))).then(exp.clone())
            .map(|(identifier, exp)| {
                TableArgTypes::WithIdentifier(identifier, exp)
            });
        let no_identifier = exp.clone().map(|exp| {
            TableArgTypes::NoIdentifier(exp)
        });
        let table = with_identifier.or(no_identifier).separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Control('[')), just(Token::Control(']')));
        table.map(|table_args| {
            let mut table = Table::new();
            for (i, table_arg) in table_args.into_iter().enumerate() {
                match table_arg {
                    TableArgTypes::WithIdentifier(identifier, pexp) => {
                        table.0.insert(TableKey::Identifier(identifier, i), pexp);
                    }
                    TableArgTypes::NoIdentifier(pexp) => {
                        table.0.insert(TableKey::NoIdentifier(i), pexp);
                    }
                }
            }
            table
        })
    });
    file_parse.map_with_span(|expr, span| (expr, span)).then_ignore(end())
}

fn ident_str(identifier: Exp) -> String {
    match identifier {
        Exp::LocalVar(str) => str,
        _ => panic!("identifier wasn't a string")
    }
}