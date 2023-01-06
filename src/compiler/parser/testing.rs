use chumsky::chain::Chain;
use proptest::proptest;
use crate::{FileHolder, ir, parse_exp, print_errors};
use crate::compiler::parser;
use crate::compiler::parser::parse_block;
use crate::Exp;
use crate::ir::LiteralValue;
#[cfg(test)]
mod test{
    use crate::compiler::parser::testing::{parse_exp_thing, parse_file};
    use crate::Exp;
    use crate::ir::{File, IdentifierT, LiteralValue, TableKeyTemp};

    #[test]
    fn literals() {
        match parse_exp_thing("1") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Integer(1)))
        }
        match parse_exp_thing("2.0") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Decimal(2.0)))
        }
        match parse_exp_thing("0.0") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Decimal(0.0)))
        }
        match parse_exp_thing("true") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Boolean(true)))
        }
        match parse_exp_thing("false") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::Boolean(false)))
        }
        match parse_exp_thing("\"some string\"") {
            None => assert!(false),
            Some(ir) => assert_eq!(ir, Exp::LiteralValue(LiteralValue::String("some string".to_string())))
        }
        match parse_exp_thing("[1, a: \"my_thing\", b]") {
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
        match parse_exp_thing("foo(bar, 1, [1, a: \"my_thing\", b])") {
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
        match parse_exp_thing(format!("{}.some_func(bar, 1)", generate_table_test()).as_str()) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn static_fn_access() {
        match parse_exp_thing(format!("{}::{}", generate_table_test(), generate_fn_test()).as_str()) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn indexed_access() {
        match parse_exp_thing(format!("{}@1", generate_table_test()).as_str()) {
            None => assert!(false),
            Some(_) => ()
        }
    }
    #[test]
    fn second_indexed_access() {
        match parse_exp_thing("[1, false, 0.1]@1") {
            None => assert!(false),
            Some(_) => {}
        }
    }
    // #[test]
    // fn empty_file_test() {
    //     match parse_file("") {
    //         None => assert!(false),
    //         Some(ir) => assert_eq!(ir, File::Empty)
    //     }
    // }
    #[test]
    fn basic_file_test() {
        match parse_file(
r#"
let x = 1;
let y = 1 + 2;
let z = my_function(a, b, c);
fn some_function(d, e, f) {
    print(d, e);
    f
}
let x = x + y;
"#              ) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn fn_closures() {
        match parse_file(r#"
        let x = 1;
        let y = 0.1;
        fn my_function<x, y>(a, b){

        }
        "#) {
            None => assert!(false),
            Some(_) => {}
        }
    }
    #[test]
    fn reassignment() {
        match parse_file(r#"
            let x = 1;
            x = 2;
        "#) {
            None => assert!(false),
            Some(_) => {}
        }
    }
}

pub fn parse_exp_thing(file: &str) -> Option<ir::Exp> {
    let file_holder = FileHolder::from(file.to_string().clone());
    let (ir, errors) = parse_exp(file.to_string());
    //print_errors(errors, file_holder);
    assert_eq!(errors.len(), 0, "{:?}", get_errors_display(errors, file_holder));
    ir
}
pub fn parse_file(file: &str) -> Option<ir::File> {
    let file_holder = FileHolder::from(file.to_string().clone());
    let (ir, errors) = parse_block(file.to_string());
    assert_eq!(errors.len(), 0, "{:?}", get_errors_display(errors, file_holder));
    ir
}
pub fn get_errors_display(errors: Vec<parser::ErrorT>, mut file_holder: FileHolder) -> String {
    let mut str = Vec::new();
    for error in errors {
        error.write(&mut file_holder, std::io::stderr());
    }
    String::from_utf8(str).unwrap()
}