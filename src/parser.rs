use chumsky::{Parser, text};
use chumsky::prelude::{end, just, none_of, one_of, recursive, Simple};
use chumsky::text::{ident, TextParser};

#[derive(Debug)]
pub struct LetStatement {
    var_name: String,
    value: Expr,
}
#[derive(Debug)]
pub enum Literal {
    I32(i32),
}
#[derive(Debug)]
pub struct FnDef {
    name: String,
    args: Vec<String>,
    body: FunctionBody
}
#[derive(Debug)]
pub struct FnCall {
    name: String,
    args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr{
    StatementAExpr(Statement, Box<Expr>),
    BracedExprBlock(BracedExprBlock),
    Literal(Literal),
    FnCall(FnCall),
}
#[derive(Debug)]
pub enum ExprBlock {
    Expr(Box<Expr>),
    Braced(BracedExprBlock),
}
#[derive(Debug)]
pub enum StatementBlock {
    Statements(Vec<Statement>),
    Braced(BracedStatementBlock),
}
#[derive(Debug)]
pub enum Statement {
    BracedBlock(BracedStatementBlock),
    ExprStatment(Box<Expr>),
    FnDef(Box<FnDef>)
}
#[derive(Debug)]
pub struct BracedStatementBlock(Box<StatementBlock>);
#[derive(Debug)]
pub struct BracedExprBlock(Box<ExprBlock>);
pub fn file_parser() -> impl Parser<char, File, Error = Simple<char>> {
    let file = recursive(|file| {
        let atom = {
            let int = text::int(10).map(|s: String| Expr::Literal(Literal::I32(s.parse().unwrap()))).padded();
            int
        };
        let expr = recursive(|expr| {
            let statement = recursive(|statement| {
                expr.clone().then_ignore(just(';')).map(|st| {
                  Statement::ExprStatment(Box::new(st))
                })
            });
            let expr_statement = statement.clone().then(expr.clone()).map(|(s, e)| {
               Expr::StatementAExpr(s, Box::new(e))
            });
            let no_statement_type = {
                atom
            };
            no_statement_type.then(just(";").ignore_then(expr)).map(|(e1, e2)| {
                Expr::StatementAExpr(Statement::ExprStatment(Box::new(e2)), Box::new(e1))
            }).or(no_statement_type)
        });
        expr.map(|expr| {
            File(FunctionBody::Expr(expr))
        })
    });
    file.then_ignore(end())
}
#[derive(Debug)]
pub struct File(FunctionBody);
#[derive(Debug)]
pub enum FunctionBody {
    Expr(Expr),
    Statement(Statement),
}
// fn hi() {
//     let int = text::int(10)
//         .map(|s: String| Expr::Literal(Literal::I32(s.parse().unwrap())))
//         .padded();
//     int
// }

// let statement = recursive(|statement| {
// let expression_statement = expr.clone().then_ignore(just(";").padded()).map(|expression| {
// Statement::ExprStatement(expression)
// });
// expression_statement
// });
// let block = recursive(|block| {
// let statements_and_expr = statement.clone().repeated().at_least(1).then(expr.clone())
// .map(|(statements, expression)| { Block::ExprBlock(ExprBlock::StatementsAndExpr {statements, expression: Box::new(expression)})});
// let single_expr = expr.clone().map(|expr| {Block::ExprBlock(ExprBlock::Expr(Box::new(expr)))});
// let statements_no_expr = statement.clone().repeated().at_least(1).map(|statements| Block::Statements(StatementsBlock(statements)));
// statements_and_expr
// });
// block.map(|a| FunctionBody(vec![a]))