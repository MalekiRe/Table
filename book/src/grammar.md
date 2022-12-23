# Grammar

Table's syntax is layed out in a BNF inspired/modified grammar format to make it easier to read and understand.

```
expr ::= 
     statement expr  |
     atom            |
     func_call       |
     '{' expr '}'    |
     

statement ::= 
        '{' statement+ '}'  |
        expr_inner ';'      |
        func_def            |
        let_statement       

expr_inner ::=
           atom          |
           func_call     |
           '{' expr '}   |

atom ::= 
     INT
     
     
let_statement ::= 
              'let' IDENT '=' expr ';'
```


## Functions

```
fn_call ::= IDENT '(' fn_call_args? ')'

fn_call_args ::= (expr ',')* (expr ','?)

fn_def ::= 'fn' IDENT '(' fn_def_args? ')' fn_body

fn_def_args ::= (IDENT ',')* (IDENT ','?)

fn_body ::= '{' statement* '}' | statement | expr
```