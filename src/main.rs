mod lexer;
mod parser2;
mod ir2;
mod wasm;
mod c;

use std::fs;
use chumsky::{Parser, Stream};
use std::ops::Range;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::Simple;
use terminal_emoji::Emoji;
use crate::lexer::{Span, Token};
use crate::parser2::ParserFile;
//
// mod parser;

fn main() {
    let src = match std::env::args().nth(1) {
        Some(filename) => std::fs::read_to_string(filename).unwrap(),
        None => {
            eprintln!("File path was not provided; loading built-in test.tbl file.");
            include_str!("test.tbl").to_string()
        }
    };

    //let parser_file = print_parse(src.clone()).unwrap();
    let c_file = c::c_compiler::generate_c_file();
    println!("here");
    c::c_compiler::compile(c_file);
    println!("am here");
    let file = fs::read("target/add.wasm").unwrap();
    wasm::wasmtime_runner(file);
    //let bytes = wasm::wasm_compiler::test();
    //wasm::wasmtime_runner(bytes);
    //println!("{:#?}", ir2::evaluate_file(parser_file));
}
fn print_parse(src: String) -> Option<ParserFile> {
    let lexer = lexer::lexer();
    let (tokens, errors) = lexer.parse_recovery(src.clone());
    match tokens {
        None => None,
        Some(tokens) => {
            let len = src.chars().count();
            let (ast, parse_errors) = parser2::file_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));;
            do_err_messages(errors, parse_errors, src.clone());
            match ast {
                None => None,
                Some(ast) => {
                    println!("{:#?}", ast.0);
                    Some(ast.0)
                }
            }
        }
    }
    // let p = parser::file_parser();
    // println!("{:#?}", p.parse(src).unwrap());
}
fn do_err_messages(errs: Vec<Simple<char>>, parse_errors: Vec<Simple<Token>>, src: String) {
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
// fn eval_parse(src: String) {
//     let p = parser::file_parser();
//     let value = ir::evaluate_file(p.parse(src).unwrap());
//     println!("{:#?}", value);
// }
