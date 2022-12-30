use chumsky::{Error, Parser, select};
use chumsky::prelude::{filter_map, just, Recursive, Simple};
use crate::parser2::Spanned;
use crate::second_attempt::ir::{Exp, File, FnCall, Value};
use crate::second_attempt::lexer::Token;
use crate::second_attempt::lexer::BooleanValues;

pub fn parse() -> impl Parser<Token, Exp, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });
    let mut exp = Recursive::declare();
    let mut fn_call = Recursive::declare();
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