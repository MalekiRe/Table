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
    Atom(Atom),                // atom !';'
    FnCall(FnCall),            // fn_call !';'
    ExprBraced(BExp),          // '{' expr '}'
    ExprBracedExpr(BExp, BExp),// '{' expr ';' '}' 'expr'
    FnDefExpr(FnDef, BExp),    // fn_def expr
    ExprExpr(BExp, BExp)       // expr ';' expr
}
impl Expr {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

pub type BStatement = Box<Statement>;
#[derive(Debug)]
pub enum Statement {
    Braced(Vec<Statement>), // '{' statement+ '}' !expr
    Expr(BExp),             // expr ';' !expr
    FnDef(FnDef)            // fn_def
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

pub fn file_parser() -> impl Parser<char, File, Error = Simple<char>> {
    let file = recursive(|file| {
        let atom = {
            let int = text::int(10).map(|s: String| Expr::Atom(Atom::Literal(Literal::I32(s.parse().unwrap())))).padded();
            int
        };
        let expr = recursive(|expr| {
            let atom_match = expr_maker!(atom, expr);
            let braced_inner = expr.clone().delimited_by(just("{").padded(), just("}").padded());
            let braced_inner_match = expr_maker!(braced_inner, expr);
            let braced_outer_match = expr.clone().then_ignore(just(";").padded()).delimited_by(just("{").padded(), just("}").padded()).then(expr).map(|(e1, e2)| {
              Expr::ExprBracedExpr(e1.boxed(), e2.boxed())
            });
            atom_match.or(braced_inner_match).or(braced_outer_match)
        });
        let statement = recursive(|statement| {
            let braced = statement.clone().repeated().at_least(1).delimited_by(just("{").padded(), just("}").padded()).then_ignore(expr.clone().not());
            let expr_stmt = expr.clone().then_ignore(just(";").padded()).then_ignore(expr.clone().not());

            let braced = braced.map(|statements| {
                Statement::Braced(statements)
            });

            let expr_stmt = expr_stmt.map(|expr| {
               Statement::Expr(expr.boxed())
            });

            braced.or(expr_stmt)
        });
        // expr.map(|expr| {
        //     File(FunctionBody::Expr(expr.boxed()))
        // }).or(statement.map(|statement|{
        //     File(FunctionBody::Statement(Box::new(statement)))
        // }))
        statement.map(|statement| {
            File(FunctionBody::Statement(Box::new(statement)))
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