use crate::compiler::parser2::scanner::{Action, Scanner};
use crate::compiler::parser::lexer::Token;

mod scanner;

fn parse_source(source: &str) {
    let mut scanner = Scanner::new(source);

}
enum Atom {
    Number(f32),
    Boolean(bool),
    String(String),
    FnCall,
}
enum Exp {
    Atom(Atom),
    BinaryExp(BinaryExp),
}
struct BinaryExp {
    lhs: Atom,
    op: BinaryOp,
    rhs: Atom,
}
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod parser_test {
    use crate::compiler::parser2::parse_source;

    #[test]
    fn basic_true() {
        let str = "true";
        parse_source(str);
    }
    #[test]
    fn let_statement() {
        let str = r#"
let x = 1;
        "#;
    }
    fn destructuring() {
        let str = r#"
let (x, y, z) = something;
        "#;
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
    table_method_call ::= exp '.' '(' call_args ')'
    table_static_call ::= exp '::' '(' call_args ')'


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