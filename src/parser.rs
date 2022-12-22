use chumsky::{Parser, text};
use chumsky::prelude::{end, just, none_of, one_of, recursive, Simple};
use chumsky::text::{ident, TextParser};

#[derive(Debug)]
pub struct LetStatement {
    var_name: String,
    value: Expr,
}
#[derive(Debug)]
pub enum Atom {
    Literal(Literal)
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

type BExp = Box<Expr>;
#[derive(Debug)]
pub enum Expr{
    Atom(Atom),                  // atom
    FnCall(FnCall),              // fn_call
    ExprBraced(BExp),            // '{' expr '}'
    StatementExp(Statement, BExp)// statement expr
}
impl Expr {
    fn boxed(self) -> BExp {
        Box::new(self)
    }
}

pub type BStatement = Box<Statement>;
#[derive(Debug)]
pub enum Statement {
    Braced(Vec<Statement>), // '{' statement+ '}'
    Expr(BExp),                        // expr ';'
    FnDef(FnDef)                       // fn_def
}

macro_rules! expr_maker {
    ($expr_name:ident, $expr:ident) => {
        {
            let semi_expr = just(";").padded().ignore_then($expr.clone()).then_ignore(just(";").not());
            $expr_name.clone().then(semi_expr).map(|(e1, e2)|{
                Expr::ExprExpr(Box::new(e2), Box::new(e1))
            }).or($expr_name.clone())
        }
    }
}
macro_rules! delim_braces {
    ($ident:expr) => {
        $ident.clone().delimited_by(just("{").padded(), just("}").padded())
    }
}

pub fn file_parser() -> impl Parser<char, File, Error = Simple<char>> {
    let file = recursive(|file| {
        let atom = {
            let int = text::int(10).map(|s: String| Expr::Atom(Atom::Literal(Literal::I32(s.parse().unwrap())))).padded();
            int
        };
        let expr = recursive(|expr| {
            let expr_inner = {
                    atom.clone()
                    .or(delim_braces!(expr))
            };
            let statement = recursive(|statement| {
                let statement_braces = delim_braces!(statement.repeated().at_least(1));
                let expr_inner = expr_inner.clone().then_ignore(just(';').padded());

                let statement_braces = statement_braces.map(|mut statement_braces| {
                   Statement::Braced(statement_braces)
                });
                let expr_inner = expr_inner.map(|exp| {
                   Statement::Expr(exp.boxed())
                });
                statement_braces.or(expr_inner)
            });
            let statement_expr = {
              statement.then(expr.clone()).map(|(s, e)| {
                  Expr::StatementExp(s, e.boxed())
              })
            };
            let expr_outer = {
                statement_expr
                    .or(atom.clone())
                    .or(delim_braces!(expr))
            };
            expr_outer
        });
        expr.map(|exp| {
            File(FunctionBody::Expr(exp.boxed()))
        })
    });
    file.then_ignore(end())
}
#[derive(Debug)]
pub struct File(FunctionBody);
#[derive(Debug)]
pub enum FunctionBody {
    Expr(BExp),
    Statement(BStatement),
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