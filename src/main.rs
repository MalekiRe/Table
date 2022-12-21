mod parser;
use chumsky::Parser;
use std::ops::Range;
use terminal_emoji::Emoji;
//
// mod parser;

fn main() {
    let src = std::fs::read_to_string("src/test.tbl").unwrap();
    let p = parser::file_parser();
    println!("{:#?}", p.parse(src).unwrap());

}
