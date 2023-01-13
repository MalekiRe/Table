use chumsky::prelude::{choice, filter, just, one_of, Simple, skip_then_retry_until, take_until};
use chumsky::{Parser, Span, text};
use chumsky::text::TextParser;
use crate::compiler::parser2::error::ErrorT;
use crate::compiler::parser::span::TSpan;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Token {
    Literal(Literal),
    Identifier(String),
    Operator(Operator),
    Control(Control),
    InferenceIdentifier,
    Keyword(Keyword),
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Keyword {
    Let,
    Match,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Literal {
    String(String),
    Number(isize, usize),
    Boolean(bool),
    Char(char),
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Control {
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    Dot,
    Comma,
    RightArrow,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operator {
    PlusEquals,
    MinusEquals,
    DivideEquals,
    MultiplyEquals,
    ModulusEquals,
    PlusPlus,
    MinusMinus,
    Range,
    Minus,
    Plus,
    Divide,
    Multiply,
    Modulus,
    LeftCaret,
    RightCaret,
    LessEqual,
    GreaterEqual,
    And,
    BitwiseAnd,
    Or,
    BitwiseOr,
    Not,
    BitwiseNot,
    Equals,
    EqualsEquals,
    NotEqual,
}
pub fn lex(source: &str) -> (Option<Vec<(Token, TSpan)>>, Vec<ErrorT>){
    let len = source.chars().count();
    let my_span = Span::new(0, len..len);
    let (tokens, mut lex_errors) = lexer()
        .parse_recovery(chumsky::Stream::from_iter(
            my_span,
            source
                .chars()
                .enumerate()
                .map(|(i, c)| (c, TSpan::new(0, i..i + 1))),
        ));
    return match tokens {
        None => (None, lex_errors),
        Some(tokens) => (Some(tokens), vec![]),
    }
}
pub fn lexer() -> impl Parser<char, Vec<(Token, TSpan)>, Error =ErrorT> {
    let escape_chars = just('\\')
        .ignore_then(just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')));

    let decimal = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)))
        .collect::<String>()
        .map(|str_num| {
            let (a, b) = str_num.split_at(str_num.find('.').unwrap());
            Token::Literal(Literal::Number(a.parse().unwrap(), b[1..b.len()].parse().unwrap()))
        })
        .labelled("number");
    let integer = text::int(10)
        .map(|str_num: String| Token::Literal(Literal::Number(str_num.parse::<isize>().unwrap(), 0)))
        .labelled("number");
    let string = just('"')
        .ignore_then(filter(|c| *c != '"' && *c != '\\').or(escape_chars).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|s| Token::Literal(Literal::String(s)))
        .labelled("string");
    let r#char = just('\'')
        .ignore_then(filter(|c| *c != '\\' && *c != '\'').or(escape_chars))
        .then_ignore(just('\''))
        .map(|c| Token::Literal(Literal::Char(c)))
        .labelled("char");
    let operator = choice((
        just("++").to(Operator::PlusPlus),
        just("--").to(Operator::MinusMinus),
        just("..").to(Operator::Range),
        just("<=").to(Operator::GreaterEqual),
        just(">=").to(Operator::LessEqual),
        just("!=").to(Operator::NotEqual),
        just("==").to(Operator::EqualsEquals),
        just("+=").to(Operator::PlusEquals),
        just("-=").to(Operator::MinusEquals),
        just("*=").to(Operator::MultiplyEquals),
        just("/=").to(Operator::DivideEquals),
        just("%=").to(Operator::ModulusEquals),
        just("=").to(Operator::Equals),
        just("+").to(Operator::Plus),
        just("-").to(Operator::Minus),
        just("*").to(Operator::Multiply),
        just("/").to(Operator::Divide),
        just("<").to(Operator::LeftCaret),
        just(">").to(Operator::RightCaret),
        just("~").to(Operator::BitwiseNot),
        just("!").to(Operator::Not),
        just("%").to(Operator::Modulus),
        )).map(Token::Operator);
    let control_char = choice((
        just('(').to(Control::LeftParen),
        just(')').to(Control::RightParen),
        just('[').to(Control::LeftSquare),
        just(']').to(Control::RightSquare),
        just('{').to(Control::LeftCurly),
        just('}').to(Control::RightCurly),
        just(',').to(Control::Comma),
        just(':').to(Control::Colon),
        just(';').to(Control::Semicolon),
        just('.').to(Control::Dot),
        )).map(Token::Control);
    let identifier = text::ident().map(|ident: String| match ident.as_str() {
        "let" => Token::Keyword(Keyword::Let),
        "match" => Token::Keyword(Keyword::Match),
        "true" => Token::Literal(Literal::Boolean(true)),
        "false" => Token::Literal(Literal::Boolean(false)),
        "_" => Token::InferenceIdentifier,
        _ => Token::Identifier(ident)
    });
    let token = decimal
        .or(integer)
        .or(string)
        .or(operator)
        .or(control_char)
        .or(identifier)
        .or(r#char)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();
    //let multiline_comment = just("/*").then(take_until(just("*/"))).padded();
    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::compiler::parser2::lexer::lex;

    #[test]
    fn first() {
        let str = r#"
        let x = 1;
        x++;
        >=
        "hi\there"
        '\n'
        "#;
        let a = lex(str);
        if a.1.len() > 0 {
            panic!("{:#?}", a.1);
        }
    }
}