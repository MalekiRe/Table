use chumsky::prelude::{filter, just, one_of, Simple, skip_then_retry_until, take_until};
use chumsky::{Parser, text};
use chumsky::text::TextParser;
use crate::compiler::parser::error::ErrorT;
use crate::compiler::parser::span::TSpan;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Token {
    Decimal(isize, usize),
    Integer(isize),
    Identifier(String),
    String(String),
    Operator(String),
    Control(char),
    InferenceIdentifier,
    Export,
    Import,
    Fn,
    Let,
    Match,
    Switch,
    Boolean(bool),
    Capture,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BooleanValues {
    True,
    False
}

pub fn lexer() -> impl Parser<char, Vec<(Token, TSpan)>, Error =ErrorT> {
    let decimal = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)))
        .collect::<String>()
        .map(|str_num| {
            let (a, b) = str_num.split_at(str_num.find('.').unwrap());
            Token::Decimal(a.parse().unwrap(), b[1..b.len()].parse().unwrap())
        });
    let integer = text::int(10)
        .map(|str_num: String| Token::Integer(str_num.parse::<isize>().unwrap()));
    let string = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String);
    let operator = one_of("@+-*/!=&|%<>")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Operator);
    let control_chars = one_of("()[]{};:,.").map(|c| Token::Control(c));
    let identifier = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "match" => Token::Match,
        "switch" => Token::Switch,
        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),
        "export" => Token::Export,
        "capture" => Token::Capture,
        "import" => Token::Import,
        "_" => Token::InferenceIdentifier,
        _ => Token::Identifier(ident)
    });
    let token = decimal
        .or(integer)
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


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Integer(number) => write!(f, "{}", number),
            Token::Decimal(a, b) => write!(f, "{}.{}", a, b),
            Token::Identifier(identifier) => write!(f, "{}", identifier),
            Token::String(string) => write!(f, "{}", string),
            Token::Operator(operator) => write!(f, "{}", operator),
            Token::Control(control) => write!(f, "{}", control),
            Token::InferenceIdentifier => write!(f, "_"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Match => write!(f, "match"),
            Token::Switch => write!(f, "switch"),
            Token::Export => write!(f, "export"),
            Token::Boolean(boolean) => {
                match boolean {
                    true => write!(f, "true"),
                    false => write!(f, "false"),
                }
            }
            Token::Capture => write!(f, "capture"),
            Token::Import => write!(f, "import"),
        }
    }
}