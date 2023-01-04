#![feature(trait_alias)]
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::Simple;
use crate::compiler::{FileHolder, ir};
use crate::compiler::parser::error::Error;
use crate::compiler::parser::lexer::Token;
use crate::compiler::parser::parse_and_lex;
use crate::ir::{Exp, LetStatement, StatementBlock};
use crate::misc::VecTuple1;
use crate::virtual_machine::bytecode;
use crate::virtual_machine::bytecode::{ALLOC_TABLE, byte_array_to_usize, Bytecode};
use crate::virtual_machine::bytecode::Bytecode::{AllocTable, LoadConstNum, LoadInstructions, PeekLocal, PopLocal, PushLocal, RegisterSet, TableGetIndex, TableSetIndex};
use crate::virtual_machine::bytecode::Bytecode::{LoadConstant, Print};
use crate::virtual_machine::chunk::Chunk;
use crate::virtual_machine::value::Value;
use crate::virtual_machine::vm::Vm;

mod virtual_machine;
mod compiler;
mod misc;

fn main() {
    let mut vm = Vm::new();
    let (chunk, constants) = test_registers();
    vm.constants = constants;
    vm.load(chunk);
    vm.run();
    lex("[a: 1, 2, true, Som_randIden212ifer: \"yo yo yo evereybody\"]".to_string());
}
pub fn lex(file: String) {
    let file_holder = FileHolder::from(file.clone());
    let (ir, errors) = parse_and_lex(file);
    print_errors(errors, file_holder);
    match ir {
        None => {}
        Some(ir) => {
            println!("ir: {:#?}", ir);
        }
    }
}
pub fn print_errors(errors: Vec<Error>, mut file_holder: FileHolder) {
    for error in errors {
        error.write(&mut file_holder, std::io::stderr());
    }
}
fn first_test_chunk() -> Chunk {
    Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(vec![LoadConstant(0), Print, AllocTable, PushLocal, LoadConstant(0), LoadConstant(1), PeekLocal(0), TableSetIndex, LoadConstant(1), PopLocal, TableGetIndex, Print].as_slice()),
        locals: vec![],
        eval_stack: vec![],
    }
}
fn second_chunk() -> (Chunk, Vec<Value>) {
    let instructions = vec![
        AllocTable,
        PushLocal,
        LoadConstNum(bytecode::LOAD_CONSTANT as usize),
        LoadConstNum(0),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(0x0),
        LoadConstNum(1),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::PRINT as usize),
        LoadConstNum(2),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::RETURN as usize),
        LoadConstNum(3),
        PeekLocal(0),
        TableSetIndex,
        PeekLocal(0),
        LoadInstructions,
    ];
    let constants = vec![
        Value::Float(69.420),
    ];
    (Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(instructions.as_slice()),
        locals: vec![],
        eval_stack: vec![]
    }, constants)
}
pub fn test_registers() -> (Chunk, Vec<Value>) {
    let instructions = vec![
        LoadConstant(0x1),
        RegisterSet(0x10),
        AllocTable,
        PushLocal,
        LoadConstNum(bytecode::REGISTER_GET as usize),
        LoadConstNum(0),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(0x10),
        LoadConstNum(1),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::PRINT as usize),
        LoadConstNum(2),
        PeekLocal(0),
        TableSetIndex,
        LoadConstNum(bytecode::RETURN as usize),
        LoadConstNum(3),
        PeekLocal(0),
        TableSetIndex,
        PeekLocal(0),
        LoadInstructions
    ];
    let constants = vec![
        Value::Float(0.01),
        Value::Float(69.420),
    ];
    (Chunk {
        ptr: 0,
        instructions: Bytecode::convert_to_bytes(instructions.as_slice()),
        locals: vec![],
        eval_stack: vec![]
    }, constants)
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