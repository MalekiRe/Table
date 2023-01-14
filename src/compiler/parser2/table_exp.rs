use chumsky::Parser;
use chumsky::prelude::{just, recursive};
use crate::compiler::parser2::error::ErrorT;
use crate::compiler::parser2::lexer::Control::{LeftSquare, RightSquare};
use crate::compiler::parser2::lexer::{Control, Token};
use crate::compiler::parser2::parser::{exp_block, fn_call, range_creation, simple_literal, table_literal};
use crate::compiler::parser2::parsing_ir;
use crate::compiler::parser2::tokens::identifier;
use crate::compiler::parser2::parsing_ir::{Exp, Statement, TableExp, TableFieldAccess, TableIndexing, TableMethodCall, TableStaticCall};
pub trait TParser<T> = chumsky::Parser<Token, T, Error =ErrorT> + Clone;
pub fn table_exp(exp: impl TParser<Exp> + 'static, statement: impl TParser<Statement> + 'static) -> impl TParser<TableExp> {
    recursive(|table_exp| {
        let table = _table(exp.clone(), statement.clone());

        let table_indexing = table_indexing(table.clone(), exp.clone(), statement.clone()).map(TableExp::TableIndexing);
        let table_field_access = table_field_access(table.clone()).map(TableExp::TableFieldAccess);
        let table_method_call = table_method_call(table.clone(), exp.clone()).map(TableExp::TableMethodCall);
        let table_static_call = table_static_call(table.clone(), exp.clone()).map(TableExp::TableStaticCall);

        table_indexing.or(table_method_call).or(table_static_call).or(table_field_access)
    })
}
/// for internal use
pub fn _table(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<Exp> {

    let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
    //let binary_exp; //TODO
    let exp_block = exp_block(statement, exp.clone()).map(Exp::ExpBlock);
    //let control_flow_exp; //TODO
    let table_literal = table_literal(exp.clone()).map(parsing_ir::Literal::TableLiteral).map(Exp::Literal);

    let table = table_literal.or(fn_call.or(exp_block));
    table
}
pub fn table_indexing(table: impl TParser<Exp>, exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<TableIndexing> {

    let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
    //let binary_exp; //TODO
    //let control_flow_exp; //TODO
    let range_creation = range_creation(exp.clone()).map(Exp::RangeCreation);
    let exp_block = exp_block(statement, exp.clone()).map(Exp::ExpBlock);
    let simple_literal = simple_literal().map(Exp::Literal);

    let index = range_creation.or(fn_call.or(exp_block.or(simple_literal)));
    table.then(index.delimited_by(just(Token::Control(LeftSquare)), just(Token::Control(RightSquare))))
        .map(|(table, index)| {
            TableIndexing {
                table: Box::new(table),
                index: Box::new(index)
            }
        })
}
pub fn table_field_access(table: impl TParser<Exp>) -> impl TParser<TableFieldAccess> {
    table.then_ignore(just(Token::Control(Control::Dot))).then(identifier())
        .map(|(table, field)| {
            TableFieldAccess {
                table: Box::new(table),
                field
            }
        })
}
pub fn table_method_call(table: impl TParser<Exp>, exp: impl TParser<Exp>) -> impl TParser<TableMethodCall> {
    table.then_ignore(just(Token::Control(Control::Dot)))
        .then(fn_call(exp))
        .map(|(table, fn_call)| {
            TableMethodCall {
                table: Box::new(table),
                fn_call
            }
        })
}
pub fn table_static_call(table: impl TParser<Exp>, exp: impl TParser<Exp>) -> impl TParser<TableStaticCall> {
    table.then_ignore(just(Token::Control(Control::Colon)).repeated().exactly(2))
        .then(fn_call(exp))
        .map(|(table, fn_call)| {
            TableStaticCall {
                table: Box::new(table),
                fn_call
            }
        })
}
