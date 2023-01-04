use chumsky::{Span, Stream, Parser, select};
use chumsky::prelude::{filter_map, just, recursive, Recursive};
use crate::compiler::parser::lexer::lexer;
use crate::{do_err_messages, ir, Token};
use crate::compiler::parser::error::{Error, Pattern};
use crate::compiler::parser::span::TSpan;
use crate::ir::{Exp, File, TableKeyTemp};

pub mod lexer;
pub mod error;
mod span;

pub trait TParser<T> = chumsky::Parser<Token, T, Error = Error> + Clone;
pub fn parse_and_lex(src: String) -> (Option<ir::Exp>, Vec<Error>) {
    let len = src.chars().count();
    let my_span = Span::new(0, len..len);
    let (tokens, mut lex_errors) = lexer()
        .parse_recovery(chumsky::Stream::from_iter(
            my_span,
            src
                .chars()
                .enumerate()
                .map(|(i, c)| (c, TSpan::new(0, i..i + 1))),
        ));
    let tokens = match tokens {
        None => return (None, lex_errors),
        Some(tokens) => tokens,
    };
    println!("tokens: {:#?}", tokens);

    let (ir, mut parser_errors) = exp().parse_recovery(chumsky::Stream::from_iter(my_span, tokens.into_iter()));
    lex_errors.append(&mut parser_errors);
    match ir {
        None => (None, lex_errors),
        Some(ir) => (Some(ir), lex_errors)
    }
}

pub fn literal_value(exp: impl TParser<ir::Exp>) -> impl TParser<ir::LiteralValue> {
    let first = select! {
        Token::Decimal(a, b) => ir::LiteralValue::Decimal(format!("{}.{}", a, b).parse().unwrap()),
        Token::Integer(a) => ir::LiteralValue::Integer(a),
        Token::String(a) => ir::LiteralValue::String(a),
        Token::Boolean(a) => ir::LiteralValue::Boolean(a),
    };
    let table_with_key = identifier().then(just(Token::Control(':')).ignore_then(exp.clone()))
        .map(|(ident, exp)| {
            TableKeyTemp {
                ident: Some(ident),
                exp: Box::new(exp)
            }
        });
    let table_without_key = exp.clone().map(|exp| {
        TableKeyTemp {
            ident: None,
            exp: Box::new(exp)
        }
    });
    let table = table_without_key.or(table_with_key).separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Control('[')), just(Token::Control(']')));
    let table = table.map(|things|{
        ir::LiteralValue::Table(things)
    });
    first.or(table).map_err(|e: Error| e.expected(Pattern::Literal))
        .labelled("literal")
}
pub fn exp() -> impl TParser<ir::Exp> {
    recursive(|exp| {
        literal_value(exp).map(|val| {
            Exp::LiteralValue(val)
        })
    }).labelled("expression")
}
pub fn identifier() -> impl TParser<ir::IdentifierT> {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => unreachable!(),
    });
    ident
}