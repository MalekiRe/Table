mod parser;
mod ir;
mod lexer;
mod parser2;

use chumsky::{Parser, Stream};
use std::ops::Range;
use terminal_emoji::Emoji;
use crate::lexer::{Span, Token};
//
// mod parser;

fn main() {
    let src = std::fs::read_to_string("src/test.tbl").unwrap();
    print_parse(src.clone());
    //eval_parse(src.clone());
}

fn print_parse(src: String) {
    let lexer = lexer::lexer();
    let (tokens, errors) = lexer.parse_recovery(src.clone());
    match tokens {
        None => {}
        Some(tokens) => {
            let len = src.chars().count();
            let (ast, errors) = parser2::file_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));;
            println!("{:#?}", ast.unwrap().0);
        }
    }
    // let p = parser::file_parser();
    // println!("{:#?}", p.parse(src).unwrap());
}
// fn eval_parse(src: String) {
//     let p = parser::file_parser();
//     let value = ir::evaluate_file(p.parse(src).unwrap());
//     println!("{:#?}", value);
// }
