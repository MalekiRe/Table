use chumsky::{Error, Parser, select};
use chumsky::prelude::{filter_map, just, Recursive, Simple};
use crate::parser2::Spanned;
use crate::second_attempt::ir::{Block, Exp, File, FnCall, LetStatement, Statement, Value};
use crate::second_attempt::lexer::Token;
use crate::second_attempt::lexer::BooleanValues;

pub fn parse() -> impl Parser<Token, Exp, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });
    let mut exp = Recursive::declare();
    let mut statement = Recursive::declare();
    let mut fn_call = Recursive::declare();
    let mut block = Recursive::declare();
    let let_statement = {
        just(Token::Let).ignore_then(ident.clone())
            .then_ignore(just(Token::Operator("=".to_string()))).then(exp.clone()).then_ignore(just(Token::Control(';')))
            .map(|(identifier, exp)|{
                Statement::LetStatement(LetStatement {
                    identifier,
                    exp: Box::new(exp)
                })
            })
    };
    statement.define({
        let_statement.or(
            exp.clone().then_ignore(just(Token::Control(';'))).map(|exp| {
                Statement::ExpStatement(Box::new(exp))
            })
        )
    });
    block.define({
       statement.clone().repeated().then(exp.clone())
           .map(|(statements, exp)| {
               Block::WithExp(statements.into_iter().map(|statement| Box::new(statement)).collect(), Box::new(exp))
           })
           .or(
               statement.clone().repeated().at_least(1)
                   .map(|statements| {
                       Block::WithoutExp(statements.into_iter().map(|statement| Box::new(statement)).collect())
                   })
           )

    });
    exp.define({
        let val = select!{
            Token::Number(n) => Exp::Value(Value::Number(n.parse().unwrap())),
            Token::String(string) => Exp::Value(Value::String(string)),
            Token::Boolean(BooleanValues::True) => unimplemented!(),
            Token::Boolean(BooleanValues::False) => unimplemented!(),
        }.labelled("value");
        let identifier = select! {
            Token::Identifier(string) => Exp::Variable(string)
        }.labelled("identifier");
        let atom = val
            .or(exp.clone().delimited_by(just(Token::Control('{')), just(Token::Control('}'))))
            .or(fn_call.clone())
            .or(identifier);
        let braced_exp = exp.clone().delimited_by(just(Token::Control('{')), just(Token::Control('}')));
        atom.or(braced_exp)
    });
    fn_call.define({
        let ident = ident.clone();
        let fn_call_args = exp.clone().separated_by(just(Token::Control(','))).allow_trailing()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')')));
        let fn_call = ident.then(fn_call_args).map(|(identifier, args)| {
            println!("{:#?}", args);
            Exp::FnCall(FnCall {
                identifier,
                args: args.into_iter().map(|arg| {Box::new(arg)}).collect()
            })
        });
        fn_call
    });
    exp
}