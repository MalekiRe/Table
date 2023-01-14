use chumsky::Parser;
use chumsky::prelude::{just, recursive};
use crate::compiler::parser2::error::ErrorT;
use crate::compiler::parser2::lexer::{Control, Token};
use crate::compiler::parser2::parser::variable;
use crate::compiler::parser2::parsing_ir::{Exp, ExpStatement, LetStatement, ReassignStatement, Statement, TableAssign, UniqueIdentTableAssign};
use crate::compiler::parser2::tokens::{colon, comma, equals, identifier, left_paren, r#let, right_paren, semicolon};

pub trait TParser<T> = chumsky::Parser<Token, T, Error =ErrorT> + Clone;

pub fn statement(exp: impl TParser<Exp> + 'static) -> impl TParser<Statement> {
    recursive(|statement| {
        let exp_statement = exp_statement(exp.clone()).map(Statement::ExpStatement);
        let let_statement = let_statement(exp.clone()).map(Statement::LetStatement);
        let reassign_statement = reassign_statement(exp.clone(), statement.clone()).map(Statement::ReassignStatement);
        let_statement.or(reassign_statement).or(exp_statement)
    })
}
pub fn exp_statement(exp: impl TParser<Exp>) -> impl TParser<ExpStatement> {
    exp.then_ignore(just(Token::Control(Control::Semicolon))).map(Box::new)
}
pub fn let_statement(exp: impl TParser<Exp>) -> impl TParser<LetStatement> {
    let uninit = r#let().ignore_then(identifier()).then_ignore(semicolon())
        .map(|identifier| {
            LetStatement::Uninitialized(identifier)
        });
    let simple = r#let().ignore_then(identifier()).then_ignore(equals())
        .then(exp.clone()).then_ignore(semicolon())
        .map(|(identifier, lhs)| {
            LetStatement::SingleAssign(identifier, Box::new(lhs))
        });

    let table_syntax_without_ident = r#let().ignore_then(
        identifier().separated_by(comma()).delimited_by(left_paren(), right_paren())
    ).then_ignore(equals()).then(exp.clone()).then_ignore(semicolon())
        .map(|(table, exp)| {
            LetStatement::Table(TableAssign {
                table,
                exp: Box::new(exp)
            })
        });
    let table_syntax_with_ident = r#let().ignore_then(
        identifier().then_ignore(colon()).then(identifier())
            .separated_by(comma()).delimited_by(left_paren(), right_paren())
    ).then_ignore(equals()).then(exp).then_ignore(semicolon())
        .map(|(table, exp)| {
            LetStatement::UniqueIdentTable(UniqueIdentTableAssign {
                table,
                exp: Box::new(exp)
            })
        });
    uninit.or(simple).or(table_syntax_with_ident).or(table_syntax_without_ident)
}
pub fn reassign_statement(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<ReassignStatement> {
    let single = variable(exp.clone(), statement.clone()).then_ignore(equals()).then(exp.clone()).then_ignore(semicolon())
        .map(|(var, exp)| {
            ReassignStatement::SingleVarAssign(var, Box::new(exp))
        });

    single
}