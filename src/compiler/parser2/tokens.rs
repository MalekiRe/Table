use chumsky::prelude::{filter_map, just};
use crate::compiler::ir::IdentifierT;
use crate::compiler::parser2::error::{ErrorKind, ErrorT, Pattern};
use crate::compiler::parser2::lexer::{Control, Keyword, Operator, Token};
use crate::compiler::parser2::parser::TParser;


pub fn colon() -> impl TParser<Token> {
    just(Token::Control(Control::Colon))
}
pub fn equals() -> impl TParser<Token> {
    just(Token::Operator(Operator::Equals))
}
pub fn left_paren() -> impl TParser<Token> {
    just(Token::Control(Control::LeftParen))
}
pub fn right_paren() -> impl TParser<Token> {
    just(Token::Control(Control::RightParen))
}
pub fn comma() -> impl TParser<Token> {
    just(Token::Control(Control::Comma))
}
pub fn r#let() -> impl TParser<Token> {
    just(Token::Keyword(Keyword::Let))
}
pub fn semicolon() -> impl TParser<Token> {
    just(Token::Control(Control::Semicolon))
}
pub fn identifier() -> impl TParser<IdentifierT> {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(ErrorT::new(ErrorKind::Unexpected(Pattern::TermIdent), span)),
    });
    ident
}