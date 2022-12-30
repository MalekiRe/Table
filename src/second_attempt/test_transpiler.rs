use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, LetStatement, Statement, Value};
use crate::second_attempt::{ir, ir3, ir_to_string_2};

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
    let value_exp = Box::new(Exp::Value(Value::Number(1.0)));
    let second_value_exp = Box::new(Exp::Value(Value::String("hi".to_string())));
    let file = File::Block(Block::WithoutExp(vec![
        Box::new(Statement::LetStatement(LetStatement { identifier: "my_var".to_string(), exp: second_value_exp })),
        Box::new(Statement::FnDef(ir::FnDef{
            identifier: "my_function".to_string(),
            args: vec![],
            body: Block::WithExp(vec![], value_exp),
            closure_idents: vec!["my_var".to_string()]
        }))
    ]));
    let generated = ir3::TranslationUnit::gen_from_file(file);
    println!("{}", generated);
}