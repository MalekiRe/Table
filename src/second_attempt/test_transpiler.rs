use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, FnCall, LetStatement, Statement, Value};
use crate::second_attempt::{ir, ir3, ir_to_string_2, parser};
use chumsky::{Parser, Stream};
use crate::second_attempt::lexer::lexer;
use crate::second_attempt::parser::do_err_messages;

pub(crate) fn test_transpiler() {
    // let file = File::Block(Block::WithExp(vec![], Box::new(Exp::Value(Value::String(String::from("yoo"))))));
    // let generated = ir_to_string_2::CFile::generate(file);
    // println!("{}", generated);
    // let file =
    // File::Block(
    //     Block::WithExp(
    //         vec![],
    //         Box::new(
    //             Exp::BinaryOperation(
    //                 BinaryOperation {
    //                     left_hand_side: Box::new(Exp::Value(Value::Number(1.0))),
    //                     operator: BinaryOperator::Add,
    //                     right_hand_side: Box::new(Exp::Value(Value::Number(1.0)))
    //                 }
    //
    //             )
    //         )
    //     )
    // );
    // let generated = ir_to_string_2::CFile::generate(file);
    // println!("{}", generated);
    // let value_exp = Box::new(Exp::Value(Value::Number(1.0)));
    // let second_value_exp = Box::new(Exp::Value(Value::String("hi".to_string())));
    // let let_statement = Box::new(Statement::LetStatement(LetStatement { identifier: "my_var".to_string(), exp: second_value_exp }));
    // let fn_def = Box::new(Statement::FnDef(ir::FnDef{
    //     identifier: "my_function".to_string(),
    //     args: vec!["foo".to_string()],
    //     body: Block::WithExp(vec![], value_exp),
    //     closure_idents: vec!["my_var".to_string()],
    //     exported: false
    // }));
    // let fn_call = Box::new(Exp::FnCall(
    //     FnCall {
    //         identifier: "my_function".to_string(),
    //         args: vec![
    //             Box::new(Exp::Value(Value::Number(2.0)))
    //         ]
    //     }
    // ));
    // let file = File::Block(Block::WithoutExp(vec![
    //     let_statement,
    //     fn_def,
    //     Box::new(Statement::ExpStatement(fn_call)),
    // ]));
    //let generated = ir3::TranslationUnit::gen_from_file(file);
    //println!("{}", generated);
}
pub(crate) fn test_parser(src: String) -> String {
    let (tokens, errors) = lexer().parse_recovery(src.clone());
    let len = src.chars().count();
    let stream = Stream::from_iter(len..len + 1, tokens.unwrap().into_iter());
    let (ast, parse_errors) = parser::parse().parse_recovery(stream);
    do_err_messages(errors, parse_errors, src.clone());
    let ast = ast.unwrap();
    //println!("{:#?}", ast);
    let generated = ir3::TranslationUnit::gen_from_file(ast);
    println!("{}", generated);
    generated
}