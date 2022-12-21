use chumsky::{Parser, text};
use chumsky::prelude::{recursive, Simple};
use chumsky::text::{ident, TextParser};

#[derive(Debug)]
enum Statement {
    ExprStatement(Expr),
    FnDef(FnDef),
    LetStatement(LetStatement)
}
#[derive(Debug)]
struct LetStatement {
    var_name: String,
    value: Expr,
}
/// this is the equiv of whats normally in a `{ }`, that whole thing including the parenthisis is a block.
#[derive(Debug)]
enum Block {
    StatementsAndExpr {
        statements: Vec<Statement>,
        expression: Expr,
    },
    Statements {
        statements: Vec<Statement>,
    },
    Expr {
        expression: Expr,
    },
    Empty,
}
#[derive(Debug)]
struct FunctionBody(pub Vec<Block>);
#[derive(Debug)]
struct File(pub FunctionBody);
#[derive(Debug)]
enum Expr {
    Literal(Literal),
    VarIdent(String),
    FnCall(FnCall),
}
#[derive(Debug)]
enum Literal {
    I32(i32),
}
#[derive(Debug)]
struct FnDef {
    name: String,
    args: Vec<String>,
    body: FunctionBody
}
#[derive(Debug)]
struct FnCall {
    name: String,
    args: Vec<Expr>,
}

fn file_parser() -> impl Parser<char, File, Error = Simple<char>> {
    //let ident = ident().padded();
    let function_body = recursive(|function_body| {
        let expr = recursive(|expr| {
            let int = text::int(10)
                .map(|s: String| Expr::Literal(Literal::I32(s.parse().unwrap())))
                .padded();
            int
        });
        let statement = recursive(|statement| {

        });
    });
    unimplemented!();
}
