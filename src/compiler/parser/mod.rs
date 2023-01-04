use chumsky::{Span, Stream, Parser, select};
use chumsky::prelude::{end, filter_map, just, recursive, Recursive, skip_until};
use crate::compiler::parser::lexer::lexer;
use crate::{do_err_messages, ir, Token};
use crate::compiler::ir::{BExp, TableOperation};
use crate::compiler::ir::TableOperation::{TableFieldAccess, TableStaticFuncCalling};
use crate::compiler::parser::error::{Error, ErrorKind, Pattern};
use crate::compiler::parser::span::TSpan;
use crate::ir::{BinaryOp, BinaryOperation, EqualityOp, Exp, ExpBlock, File, FnCall, LiteralValue, MathOp, TableKeyTemp, UnaryPrefixOp, UnaryPrefixOperation};

pub mod lexer;
pub mod error;
mod span;

pub trait TParser<T> = chumsky::Parser<Token, T, Error = Error> + Clone;
pub fn parse_and_lex(src: String) -> (Option<ir::Exp>, Vec<Error>) {
    let len = src.chars().count();
    let my_span = Span::new(0, len..len);
    let (tokens, mut lex_errors) = lexer()
        .parse_recovery(chumsky::Stream::from_iter(
            my_span,
            src
                .chars()
                .enumerate()
                .map(|(i, c)| (c, TSpan::new(0, i..i + 1))),
        ));
    let tokens = match tokens {
        None => return (None, lex_errors),
        Some(tokens) => tokens,
    };
    println!("tokens: {:#?}", tokens);

    let (ir, mut parser_errors) = exp().then_ignore(end()).parse_recovery(chumsky::Stream::from_iter(my_span, tokens.into_iter()));
    lex_errors.append(&mut parser_errors);
    match ir {
        None => (None, lex_errors),
        Some(ir) => (Some(ir), lex_errors)
    }
}

pub fn literal_value(exp: impl TParser<ir::Exp>) -> impl TParser<ir::LiteralValue> {
    let first = select! {
        Token::Decimal(a, b) => ir::LiteralValue::Decimal(format!("{}.{}", a, b).parse().unwrap()),
        Token::Integer(a) => ir::LiteralValue::Integer(a),
        Token::String(a) => ir::LiteralValue::String(a),
        Token::Boolean(a) => ir::LiteralValue::Boolean(a),
    };
    let table_with_key = identifier().then(just(Token::Control(':')).ignore_then(exp.clone()))
        .map(|(ident, exp)| {
            TableKeyTemp {
                ident: Some(ident),
                exp: Box::new(exp)
            }
        });
    let table_without_key = exp.clone().map(|exp| {
        TableKeyTemp {
            ident: None,
            exp: Box::new(exp)
        }
    });
    let table = table_with_key.or(table_without_key).separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Control('[')), just(Token::Control(']')));
    let table = table.map(|things|{
        ir::LiteralValue::Table(things)
    });
    first.or(table).map_err(|e: Error| e.expected(Pattern::Literal))
        .labelled("literal")
}
pub fn fn_call(exp: impl TParser<Exp>) -> impl TParser<ir::FnCall> {
    identifier().then(
        exp.clone().separated_by(just(Token::Control(','))).allow_trailing()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')')))
    ).map(|(identifier, exps)| {
        FnCall {
            identifier,
            args: exps.into_iter().map(|exp| Box::new(exp)).collect()
        }
    })
}
pub fn unary_prefix_operation(exp: impl TParser<Exp>) -> impl TParser<UnaryPrefixOperation> {
    just(Token::Operator("!".to_string())).ignore_then(exp)
        .map(|exp| {
            UnaryPrefixOperation {
                op: UnaryPrefixOp::Not,
                exp: Box::new(exp),
            }
        })
}
// pub fn exp_block(exp: impl TParser<Exp>) -> impl TParser<ExpBlock> {
//     unimplemented!()
// }
pub fn exp() -> impl TParser<ir::Exp> {
    recursive(|exp| {
        literal_value(exp.clone()).map(|val| {
            Exp::LiteralValue(val)
        })
            .or(
                fn_call(exp.clone())
                    .map(|fn_call| {
                        Exp::FnCall(fn_call)
                    })
            )
            .or(identifier().map(|identifier| {
                Exp::Variable(identifier)
            }))
            .or(unary_prefix_operation(exp.clone()).map(|op| {
                Exp::UnaryPrefixOperation(op)
            }))
            .or(table_operation(exp.clone()).map(|table_op| {
                Exp::TableOperation(table_op)
            }))
            .or(binary_operation(exp.clone()).map(|op| {
                Exp::BinaryOperation(op)
            }))
    }).labelled("expression")
}
pub fn table_operation(exp: impl TParser<Exp>) -> impl TParser<TableOperation> {
    let table_indexing = exp.clone().then(exp.clone()).map(|(table, index)| {
       TableOperation::TableIndexing { table: Box::new(table), index: Box::new(index) }
    });
    let table_method_calling = exp.clone().then_ignore(just(Token::Control('.'))).then(fn_call(exp.clone())).map(|(table, method)|{
       TableOperation::TableMethodCalling {
           table: Box::new(table),
           method
       }
    });
    let table_field_access = exp.clone().then_ignore(just(Token::Control('.'))).then(identifier()).map(|(table, field)| {
       TableFieldAccess {
           table: Box::new(table),
           field
       }
    });
    let table_static_fn_calling = exp.clone().then_ignore(just(Token::Control(':'))).then_ignore(just(Token::Control(':'))).then(fn_call(exp.clone())).map(|(table, method)|{
       TableStaticFuncCalling {
           table: Box::new(table),
           method
       }
    });
    table_indexing.or(table_method_calling).or(table_field_access).or(table_static_fn_calling)
}
pub fn binary_operation(exp: impl TParser<Exp>) -> impl TParser<BinaryOperation> {
    //TODO Actual order of operations
    let binary_op = select!{
        Token::Operator(operator) => {
            match operator.as_str() {
                "+" => BinaryOp::Math(MathOp::Add),
                "-" => BinaryOp::Math(MathOp::Subtract),
                "/" => BinaryOp::Math(MathOp::Divide),
                "*" => BinaryOp::Math(MathOp::Multiply),
                "%" => BinaryOp::Math(MathOp::Modulo),
                "+=" => BinaryOp::Math(MathOp::AddEqual),
                "-=" => BinaryOp::Math(MathOp::MinusEqual),
                "/=" => BinaryOp::Math(MathOp::DivideEqual),
                "*=" => BinaryOp::Math(MathOp::MultiplyEqual),
                "%=" => BinaryOp::Math(MathOp::ModuloEqual),

                "==" => BinaryOp::Equality(EqualityOp::EqualsEquals),
                "!=" => BinaryOp::Equality(EqualityOp::EqualsNot),
                ">=" => BinaryOp::Equality(EqualityOp::EqualsGreater),
                "<=" => BinaryOp::Equality(EqualityOp::EqualsLess),
                ">" => BinaryOp::Equality(EqualityOp::Greater),
                "<" => BinaryOp::Equality(EqualityOp::Less),
                "&" => BinaryOp::Equality(EqualityOp::And),
                "|" => BinaryOp::Equality(EqualityOp::Or),
                &_ => panic!("how?"),
            }
        }
    };
    exp.clone().then(binary_op).then(exp)
        .map(|((lhs, op), rhs)| {
            BinaryOperation {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs)
            }
        })
}
pub fn identifier() -> impl TParser<ir::IdentifierT> {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(error::Error::new(ErrorKind::Unexpected(Pattern::TermIdent), span)),
    });
    ident
}