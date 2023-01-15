use crate::compiler::parser2::lexer::Token;
use crate::compiler::parser::span::TSpan;


mod lexer;
mod parser;
mod error;
mod tokens;
mod table_exp;
mod parsing_ir;
mod statement;
mod vm2;
mod ir_bytecode_compiler;


#[cfg(test)]
mod parser_test {
    use crate::compiler::parser2::parser::parse;

    #[test]
    fn basic_literal() {
        parse("true").unwrap();
        parse("false").unwrap();
        parse("'a'").unwrap();
        parse(r#"'\n'"#).unwrap();
        parse("1").unwrap();
        parse("1.2").unwrap();
        parse(r#""hi""#).unwrap();
        parse(r#""\"hi\"""#).unwrap();
    }
    #[test]
    fn statement_exp() {
        parse("1").unwrap();
        parse("1;").unwrap();
        parse("1;1").unwrap();
    }
    #[test]
    fn exp_block() {
        parse("{1; 1; 1}").unwrap();
    }
    #[test]
    fn fn_call() {
        parse("foo()").unwrap();
        parse("my_func(1, 2, 3)").unwrap();
        parse("foo(bar(1), 2, 3)").unwrap();
    }
    #[test]
    fn range_creation() {
        parse("0..2").unwrap();
        parse("foo(1)..bar()").unwrap();
        parse("foo(1..2)").unwrap();
        parse("foo(foo(1..2)..2)..bar()").unwrap();
    }
    #[test]
    fn fn_dec() {
        parse("(a, b, c) -> 1").unwrap();
        parse("() -> print(\"hi\");").unwrap();
        parse("() -> foo()").unwrap();
    }
    #[test]
    fn empty_file() {
        parse("").unwrap();
    }
    #[test]
    fn table_literal() {
        parse("(1, 2, 3)").unwrap();
        parse("(a: 1, 2, b: \"hiii\")").unwrap();
    }
    #[test]
    fn table_indexing() {
        parse(r#"
        (a: 1, 2, b: "hiii")[1]
        "#).unwrap();
        parse("foo(1, 2)[0]").unwrap();
        parse("foo()[1..3]").unwrap();
    }
    #[test]
    fn table_field_access() {
        parse("(a: 1, 2, b: \"hii\").a").unwrap();
        parse("foo(1,2).b").unwrap();
        parse("(1, 2, 3).a").unwrap();
    }
    #[test]
    fn table_method_call() {
        parse("(a: 1, 2, b: \"hii\").a()").unwrap();
        parse("foo(1,2).b(1, 2, 3)").unwrap();
        parse("(1, 2, 3).a()").unwrap();
    }
    #[test]
    fn table_static_call() {
        parse("(a: 1, 2, b: \"hii\")::a()").unwrap();
        parse("foo(1,2)::b(1, 2, 3)").unwrap();
        parse("(1, 2, 3)::a()").unwrap();
    }
    #[test]
    fn uninit_let() {
        parse("let x;").unwrap();
    }
    #[test]
    fn simple_let() {
        parse("let x = foo();").unwrap();
        parse("let my_var = (1, a: 2, 3)[1..2];").unwrap();
    }
    #[test]
    fn complex_let_no_ident() {
        parse("let (a, b, c) = foo();").unwrap();
        parse("let (something, another) = (1, a: 2, 3)[1..2];").unwrap();
    }
    #[test]
    fn complex_let_with_ident() {
        parse("let (a: z, b: y, c: z) = foo();").unwrap();
        let y = parse("let (heyy: something, yooo: another) = (1, a: 2, 3)[1..2];").unwrap();
    }
    #[test]
    fn simple_reassignment() {
        parse("x = foo();").unwrap();
    }
    #[test]
    fn simple_statement_then_exp() {
        parse("let x = 1; x").unwrap();
    }

}
/*
some example code
let x = 1;
let some = <x>(y, z) -> {

}

let print_hi = () -> print("hi");

print_hi();

match x {
    1 -> print("is one")
    2 -> print("hello")
};

let val = switch {
    x == 1 -> print("hi"); break with 10;
    y == 2 -> {
        print("yo");
    }
    z == 3 -> {
        break with "hi";
    }
    _ -> break with 0;
};

if x print('hi');


*/
/*
The Grammar

block ::= exp | statement
file ::= block | EMPTY

macro_call ::=
           IDENT '!' '(' fn_call_args  ') |
           exp '.' '!' '(' fn_call_args ')'

variable   ::= IDENT | table_indexing | table_field_access

literal_code ::= '#{' ANYTHING '}#'
binary_exp ::= exp BINARY_OP exp
fn_call    ::= IDENT '(' call_args ')'
fn_dec     ::= ('<' dec_args '>')? '(' dec_args ')' '->' block
exp_block  ::= '{' statement* exp '}'
range_creation ::= exp '..' exp

exp ::=
    binary_exp |
    exp_block  |
    fn_call    |
    fn_dec     |
    table_exp  |
    control_flow_exp |
    range_creation   |
    macro_call       |
    literal_code     |
    literal          |
    variable_identifier

literal ::=
        "'" CHAR? "'" |
        '"' CHAR* '"' |
        INTEGER ('.' INTEGER)?
        'true' | 'false' |
        table_literal    |

CHAR ::= A-z
INTEGER ::= 0-9

table_literal ::=
              '(' (ident ':')? exp ')' // trailing commas

control_flow_exp ::=
                 match_exp  |
                 loop_exp   |
                 for_exp    |

    for_exp   ::= 'for' IDENT 'in' exp block

    loop_exp  ::= 'loop' block

    match_exp ::= 'match' exp? '{' match_body* '}'

    match_head ::= exp | exp ';' | exp ';' exp

    match_body ::= match_head ('->' | '=>' ) block

table_exp ::=
          table_indexing     |
          table_method_call  |
          table_static_call  |
          table_field_access |

    table_indexing ::= exp '[' exp ']'
    table_field_access ::= exp '.' IDENT
    table_method_call ::= exp '.' IDENT '(' call_args ')'
    table_static_call ::= exp '::' IDENT '(' call_args ')'


statement ::=
          statement_block     |
          let_statement       |
          reassign_statement  |
          break_statement     |
          if_statement        |
          return_statement    |
          exp ';'             |
          macro_call          |

    return_statement ::=
                     'return' ';'
                     'return' exp ';'

    break_statement ::=
              'break' NUMBER 'with' exp ';' |
              'break' 'with' exp ';' |
              'break' NUMBER ';' |
              'break' ';'

    statement_block ::= '{' statement* '}'

    reassign_statement ::=
                       variable '=' exp ';'                |
                       '(' ('let' IDENT) | variable ')' '=' exp ';' | //trailing commas
                       '(' IDENT ':' ('let' IDENT) | variable ')' '=' exp ';' //trailing commas

    let_statement ::=
                  'let' IDENT ';' |
                  'let' IDENT '=' exp ';' |
                  'let' '(' IDENT ')' '=' exp ';' | //trailing commas
                  'let' '(' IDENT ':' IDENT ')' '=' exp ';' | //trailing commas

    if_statement ::= 'if' exp block ';'
 */