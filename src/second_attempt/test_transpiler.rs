use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, Statement, Value};
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
    let file = File::Block(Block::WithoutExp(vec![
        Box::new(Statement::FnDef(ir::FnDef{
            identifier: "".to_string(),
            args: vec![],
            body: Block::WithExp(vec![], value_exp),
            closure_idents: vec![]
        }))
    ]));
    let generated = ir3::TranslationUnit::gen_from_file(file);
    println!("{}", generated);
}