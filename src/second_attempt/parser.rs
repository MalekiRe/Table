use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{Error, Parser, select};
use chumsky::prelude::{empty, end, filter_map, just, Recursive, Simple};
use crate::parser2::Spanned;
use crate::second_attempt::ir::{Block, Exp, File, FnCall, FnDef, LetStatement, Statement, Value};
use crate::second_attempt::lexer::Token;
use crate::second_attempt::lexer::BooleanValues;

pub fn parse() -> impl Parser<Token, Block, Error = Simple<Token>> + Clone {
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
    let fn_def = {
        let fn_def_args = ident.clone().separated_by(just(Token::Control(','))).allow_trailing();
        let capture_clause = just(Token::Capture).ignore_then(
            ident.clone().separated_by(just(Token::Control(','))).allow_trailing()
        );
        just(Token::Fn)
            .ignore_then(ident.clone())
            .then(fn_def_args.clone().delimited_by(just(Token::Control('(')), just(Token::Control(')'))))
            .then(capture_clause)
            .then(block.clone())
            .map(|(((identifier, args), capture_clause), block)| {
                FnDef {
                    identifier,
                    args,
                    body: block,
                    closure_idents: capture_clause,
                    exported: false
                }
            })
    };
    statement.define({
        let statement_block = statement.clone().repeated().delimited_by(just(Token::Control('{')), just(Token::Control('}')))
            .map(|statements| {
               Statement::Block(statements.into_iter().map(|statement| Box::new(statement)).collect())
            });
        let_statement.or(
            exp.clone().then_ignore(just(Token::Control(';'))).map(|exp| {
                Statement::ExpStatement(Box::new(exp))
            })
                .or(statement_block)
                .or(fn_def.clone().map(|fn_def| {
                    Statement::FnDef(fn_def)
                }))
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
        let exp_block = statement.clone().repeated().then(exp.clone()).delimited_by(just(Token::Control('{')), just(Token::Control('}')))
            .map(|(statements, exp)| {
               Exp::Block(statements.into_iter().map(|statement| Box::new(statement)).collect(), Box::new(exp))
            });
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
        //let braced_exp = exp.clone().delimited_by(just(Token::Control('{')), just(Token::Control('}')));
        atom.or(exp_block)
    });
    fn_call.define({
        let ident = ident.clone();
        let fn_call_args = exp.clone().separated_by(just(Token::Control(','))).allow_trailing()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')')));
        let fn_call = ident.then(fn_call_args).map(|(identifier, args)| {
            Exp::FnCall(FnCall {
                identifier,
                args: args.into_iter().map(|arg| {Box::new(arg)}).collect()
            })
        });
        fn_call
    });

    block.then_ignore(end())
}


pub fn do_err_messages(errs: Vec<Simple<char>>, parse_errors: Vec<Simple<Token>>, src: String) {
    errs.into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .chain(parse_errors.into_iter().map(|e| e.map(|tok| tok.to_string())))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
}