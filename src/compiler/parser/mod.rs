use chumsky::{Span, Stream, Parser, select};
use chumsky::prelude::{end, filter_map, just, recursive, Recursive, skip_until};
use chumsky::primitive::empty;
use crate::compiler::parser::lexer::lexer;
use crate::{do_err_messages, ir, LetStatement, StatementBlock, Token, VecTuple1};
use crate::compiler::ir::{BExp, TableOperation};
use crate::compiler::ir::TableOperation::{TableFieldAccess, TableStaticFuncCalling};
use crate::compiler::parser;
use crate::compiler::parser::error::{ErrorT, ErrorKind, Pattern};
use crate::compiler::parser::span::TSpan;
use crate::ir::{BinaryOp, BinaryOperation, Block, BStatement, EqualityOp, Exp, ExpBlock, File, FnBody, FnCall, FnDec, FnImport, LiteralValue, MathOp, OptionalBlock, OptionalStatementBlock, ReassignmentStatement, Statement, TableKeyTemp, UnaryPrefixOp, UnaryPrefixOperation};

pub mod lexer;
pub mod error;
mod span;
mod testing;

pub trait TParser<T> = chumsky::Parser<Token, T, Error =ErrorT> + Clone;
pub fn parse_exp(src: String) -> (Option<ir::Exp>, Vec<ErrorT>) {
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
    //println!("tokens: {:#?}", tokens);

    let (ir, mut parser_errors) = exp().then_ignore(end()).parse_recovery(chumsky::Stream::from_iter(my_span, tokens.into_iter()));
    lex_errors.append(&mut parser_errors);
    match ir {
        None => (None, lex_errors),
        Some(ir) => (Some(ir), lex_errors)
    }
}
pub fn parse_block(src: String) -> (Option<ir::File>, Vec<ErrorT>) {
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
    //println!("tokens: {:#?}", tokens);

    let (ir, mut parser_errors) = file(exp()).parse_recovery(chumsky::Stream::from_iter(my_span, tokens.into_iter()));
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
    first.or(table).map_err(|e: ErrorT| e.expected(Pattern::Literal))
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
pub fn exp() -> impl TParser<ir::Exp> {
    recursive(|exp| {
        let statement = statement(exp.clone());
        let literal_value = literal_value(exp.clone()).map(Exp::LiteralValue);
        let variable = identifier().map(Exp::Variable);
        let unary_prefix_operation = unary_prefix_operation(exp.clone()).map(Exp::UnaryPrefixOperation);
        let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
        let table_operation = table_operation(exp.clone()).map(Exp::TableOperation);
        let exp_block = exp_block(exp.clone(), statement.clone()).map(Exp::ExpBlock);

        let pre_ops_exp = table_operation.clone()
            .or(literal_value.clone())
            .or(fn_call.clone())
            .or(variable.clone())
            .or(unary_prefix_operation.clone())
            .or(exp_block.clone());

        let binary_operation = binary_operation(pre_ops_exp.clone()).map(Exp::BinaryOperation);
        binary_operation.or(pre_ops_exp)

    }).labelled("expression")
}
pub fn table_operation(exp: impl TParser<Exp>) -> impl TParser<TableOperation> {
    let fn_call_exp = fn_call(exp.clone()).map(Exp::FnCall);
    let literal_value = literal_value(exp.clone()).map(Exp::LiteralValue);
    let variable = identifier().map(Exp::Variable);
    let table = fn_call_exp.clone().or(literal_value).clone().or(variable.clone());
    let table_indexing = table.clone().then_ignore(just(Token::Control('@'))).then(exp.clone()).map(|(table, index)| {
       TableOperation::TableIndexing { table: Box::new(table), index: Box::new(index) }
    });
    let table_method_calling = table.clone().then_ignore(just(Token::Control('.'))).then(fn_call(exp.clone())).map(|(table, method)|{
       TableOperation::TableMethodCalling {
           table: Box::new(table),
           method
       }
    });
    let table_field_access = table.clone().then_ignore(just(Token::Control('.'))).then(identifier()).map(|(table, field)| {
       TableFieldAccess {
           table: Box::new(table),
           field
       }
    });
    let table_static_fn_calling = table.clone().then_ignore(just(Token::Control(':'))).then_ignore(just(Token::Control(':'))).then(fn_call(exp.clone())).map(|(table, method)|{
       TableStaticFuncCalling {
           table: Box::new(table),
           method
       }
    });
    table_indexing
         .or(table_method_calling)
         .or(table_field_access)
         .or(table_static_fn_calling)
}
macro_rules! op_macro{
    ($op:expr, $term:expr) => {
        Token::Operator(string) if string.as_str() == $op => $term
    }
}
pub fn binary_operation(exp: impl TParser<Exp>) -> impl TParser<BinaryOperation> {
    //TODO Actual order of operations
    // let binary_op = select!{
    //     Token::Operator(operator) => {
    //         match operator.as_str() {
    //             "+" => BinaryOp::Math(MathOp::Add),
    //             "-" => BinaryOp::Math(MathOp::Subtract),
    //             "/" => BinaryOp::Math(MathOp::Divide),
    //             "*" => BinaryOp::Math(MathOp::Multiply),
    //             "%" => BinaryOp::Math(MathOp::Modulo),
    //             "+=" => BinaryOp::Math(MathOp::AddEqual),
    //             "-=" => BinaryOp::Math(MathOp::MinusEqual),
    //             "/=" => BinaryOp::Math(MathOp::DivideEqual),
    //             "*=" => BinaryOp::Math(MathOp::MultiplyEqual),
    //             "%=" => BinaryOp::Math(MathOp::ModuloEqual),
    //
    //             "==" => BinaryOp::Equality(EqualityOp::EqualsEquals),
    //             "!=" => BinaryOp::Equality(EqualityOp::EqualsNot),
    //             ">=" => BinaryOp::Equality(EqualityOp::EqualsGreater),
    //             "<=" => BinaryOp::Equality(EqualityOp::EqualsLess),
    //             ">" => BinaryOp::Equality(EqualityOp::Greater),
    //             "<" => BinaryOp::Equality(EqualityOp::Less),
    //             "&" => BinaryOp::Equality(EqualityOp::And),
    //             "|" => BinaryOp::Equality(EqualityOp::Or),
    //             &_ => panic!("impossible"),
    //         }
    //     }
    // };
    let binary_op = select! {
        Token::Operator(string) if string.as_str() == "+" => BinaryOp::Math(MathOp::Add),
        Token::Operator(string) if string.as_str() == "-" => BinaryOp::Math(MathOp::Subtract),
    }.map_err(|e: ErrorT| e.expected(Pattern::Token(Token::Operator("+-=/*&|".to_string()))));
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
        _ => Err(error::ErrorT::new(ErrorKind::Unexpected(Pattern::TermIdent), span)),
    });
    ident
}
//STATEMENTS and related stuff mostly below Expressions and related stuff mostly above
pub fn statement(exp: impl TParser<ir::Exp> + 'static) -> impl TParser<ir::Statement> {
    recursive(|statement| {
        exp_statement(exp.clone())
            .or(fn_dec(exp.clone(), statement.clone()).map(Statement::FnDec))
            .or(fn_import().map(Statement::FnImport))
            .or(let_statement(exp.clone()).map(Statement::LetStatement))
            .or(reassignment_statement(exp.clone()).map(Statement::ReassignmentStatement))
    })
}
pub fn let_statement(exp: impl TParser<Exp>) -> impl TParser<LetStatement> {
    //TODO allow destructuring for tables here, and also allow multiple things to declare in 1 statement.
    just(Token::Let)
        .ignore_then(identifier())
        .then_ignore(just(Token::Operator("=".to_string())))
        .then(exp)
        .then_ignore(just(Token::Control(';')))
        .map(|(identifier, exp)| {
            LetStatement {
                identifier,
                lhs: Box::new(exp),
            }
        })
}
pub fn reassignment_statement(exp: impl TParser<Exp>) -> impl TParser<ReassignmentStatement> {
    identifier().then_ignore(just(Token::Operator("=".to_string())))
        .then(exp)
        .then_ignore(just(Token::Control(';')))
        .map(|(identifier, exp)| {
          ReassignmentStatement {
              identifier,
              lhs: Box::new(exp)
          }
        })
}
pub fn fn_import() -> impl TParser<FnImport> {
    just(Token::Import)
        .ignore_then(just(Token::Fn))
        .ignore_then(identifier())
        .then(identifier().separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Control('(')), just(Token::Control(')'))))
        .then_ignore(just(Token::Control(';')))
        .map(|(identifier, args)| {
            FnImport {
                identifier,
                args
            }
        })
}
pub fn exp_statement(exp: impl TParser<Exp>) -> impl TParser<Statement> {
    exp.clone().then_ignore(just(Token::Control(';')))
        .map(Box::new).map(Statement::ExpStatement)
}
pub fn fn_dec(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<FnDec> {
    let latter_half = just(Token::Fn)
        .ignore_then(identifier())
        .then(identifier().separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Operator("<".to_string())), just(Token::Operator(">".to_string()))).or_not())
        .then(identifier().separated_by(just(Token::Control(','))).allow_trailing().delimited_by(just(Token::Control('(')), just(Token::Control(')'))))
        .then(fn_body(exp.clone(), statement.clone()));
    just(Token::Export).or_not().then(latter_half.clone())
        .map(|(possible_export, (((identifier, closed_args), args), block))| {
            FnDec {
                identifier,
                args,
                closed_args: match closed_args {
                    None => vec![],
                    Some(a) => a,
                },
                body: block,
                exported: possible_export.is_some()
            }
        })
}
//BLOCKS AND BODIES
pub fn fn_body(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<FnBody> {
    exp.clone().map(Box::new).map(FnBody::Exp)
        .or(statement.clone().map(Box::new).map(FnBody::Statement))
        .or(optional_block(exp, statement).map(FnBody::OptionalBlock))
}
pub fn exp_block(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<ExpBlock> {
    statement.repeated().then(exp).delimited_by(just(Token::Control('{')), just(Token::Control('}')))
        .map(|(statements, exp)| {
            ExpBlock(statements.into_iter().map(Box::new).collect(), Box::new(exp))
        })
}
pub fn statement_block(statement: impl TParser<Statement>) -> impl TParser<StatementBlock> {
    statement.repeated().at_least(1).delimited_by(just(Token::Control('{')), just(Token::Control('}'))).map(|statements| {
      StatementBlock(VecTuple1::try_from(statements.into_iter().map(Box::new).collect::<Vec<BStatement>>()).unwrap())
    })
}
pub fn block(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<Block> {
    exp_block(exp.clone(), statement.clone())
        .map(Block::ExpBlock)
        .or(statement_block(statement.clone()).map(Block::StatementBlock))
}
pub fn optional_statement_block(statement: impl TParser<Statement>) -> impl TParser<OptionalStatementBlock> {
    statement_block(statement).map(OptionalStatementBlock::StatementBlock)
        .or(just(Token::Control('{')).then_ignore(just(Token::Control('}'))).map(|_| OptionalStatementBlock::Empty))
}
pub fn optional_block(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<OptionalBlock> {
    exp_block(exp.clone(), statement.clone())
        .map(OptionalBlock::ExpBlock)
        .or(optional_statement_block(statement).map(OptionalBlock::OptionalStatementBlock))
}
pub fn file(exp: impl TParser<Exp> + 'static) -> impl TParser<File> {
    let statement = statement(exp.clone());
    let statements = statement.clone().repeated().at_least(1).map(|statements| VecTuple1::try_from(statements.into_iter().map(Box::new).collect::<Vec<BStatement>>()).unwrap());
    let statements = statements.then_ignore(end()).map(File::JustStatements);
    let exp_statements = statement.clone().repeated().then(exp.clone())
        .then_ignore(end())
        .map(|(statements, exp)| {
           File::StatementExp(statements.into_iter().map(Box::new).collect(), Box::new(exp))
        });
    statements.or(exp_statements)
}