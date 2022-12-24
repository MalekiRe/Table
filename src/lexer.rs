use chumsky::error::Simple;
use chumsky::{Parser, text};
use chumsky::prelude::{filter, just, one_of, skip_then_retry_until, take_until};
use chumsky::text::TextParser;
use crate::parser::File;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
    String(String),
    Operator(String),
    Control(char),
    Fn,
    Let,
    Match,
    Switch,
    Boolean(BooleanValues),
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BooleanValues {
    True,
    False
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let number = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Number);
    let string = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String);
    let operator = one_of("+-*/!=&|")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Operator);
    let control_chars = one_of("()[]{};,").map(|c| Token::Control(c));
    let identifier = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "match" => Token::Match,
        "switch" => Token::Switch,
        "true" => Token::Boolean(BooleanValues::True),
        "false" => Token::Boolean(BooleanValues::False),
        _ => Token::Identifier(ident)
    });
    let token = number
        .or(string)
        .or(operator)
        .or(control_chars)
        .or(identifier)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}