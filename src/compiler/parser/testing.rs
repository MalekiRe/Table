use chumsky::chain::Chain;
use proptest::proptest;
use crate::{FileHolder, ir, parse_and_lex, print_errors};
use crate::compiler::parser;
use crate::Exp;
use crate::ir::LiteralValue;
#[cfg(test)]
mod test{
    use crate::compiler::parser::testing::parse;
    use crate::Exp;
    use crate::ir::{IdentifierT, LiteralValue, TableKeyTemp};

    #[test]
    fn literals() {
        match parse("1") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Integer(1)))
        }
        match parse("2.0") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Decimal(2.0)))
        }
        match parse("0.0") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Decimal(0.0)))
        }
        match parse("true") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Boolean(true)))
        }
        match parse("false") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Boolean(false)))
        }
        match parse("\"some string\"") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::String("some string".to_string())))
        }
        match parse("[1, a: \"my_thing\", b]") {
            None => {}
            Some(ir) => {
                assert_eq!(ir, Exp::LiteralValue(LiteralValue::Table(
                    vec![TableKeyTemp{ ident: None, exp: Box::new(Exp::LiteralValue(LiteralValue::Integer(1))) },
                        TableKeyTemp{ ident: Some("a".to_string()), exp: Box::new(Exp::LiteralValue(LiteralValue::String(String::from("my_thing"))))},
                        TableKeyTemp{ ident: None, exp: Box::new(Exp::Variable(String::from("b"))) }
                    ]
                )))
            }
        }
    }
    #[test]
    fn fn_call() {
        match parse("foo(bar, 1, [1, a: \"my_thing\", b])") {
            None => assert!(false),
            Some(ir) => {
                match ir {
                    Exp::FnCall(_) => {}
                    _ => assert!(false),
                }
            }
        }
    }
    fn generate_table_test() -> String {
        String::from("foo(bar, 1, [1, a: \"my_thing\", b])")
    }
    fn generate_fn_test() -> String {
        String::from("foo(bar, 1, [1, a: \"my_thing\", b])")
    }
    #[test]
    fn method_access() {
        match parse(format!("{}.some_func(bar, 1)", generate_table_test()).as_str()) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn static_fn_access() {
        match parse(format!("{}::{}", generate_table_test(), generate_fn_test()).as_str()) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn exp_block() {

    }
}

pub fn parse(file: &str) -> Option<ir::Exp> {
    let file_holder = FileHolder::from(file.to_string().clone());
    let (ir, errors) = parse_and_lex(file.to_string());
    //print_errors(errors, file_holder);
    assert_eq!(errors.len(), 0, "{:?}", get_errors_display(errors, file_holder));
    ir
}
pub fn get_errors_display(errors: Vec<parser::Error>, mut file_holder: FileHolder) -> String {
    let mut str = Vec::new();
    for error in errors {
        error.write(&mut file_holder, std::io::stderr());
    }
    String::from_utf8(str).unwrap()
}