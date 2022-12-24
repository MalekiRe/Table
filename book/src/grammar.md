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
     
```
## Let Statement
```
ident_thing ::= IDENT | '_' 

destructure_table ::= '[' (ident_thing ',')* (ident_thing ','?) ']'

let_statement ::= 'let' ( (destructure_table | ident_thing) '=' expr ',' )* (destructure_table | ident_thing) '=' expr ','? ';' 

```

## Table
```

named_val ::= IDENT ':' expr

existing_val ::= IDENT

table_val ::= named_val | expr | existing_val

table_construction ::= '[' (table_val ',')* (table_val ','?)? ']'

index_table ::= IDENT '[' expr ']'

access_table ::= IDENT '.' IDENT

```

## Functions

```
fn_call ::= IDENT '(' fn_call_args? ')'

fn_call_args ::= (expr ',')* (expr ','?)

fn_def ::= 'fn' IDENT '(' fn_def_args? ')' fn_body

fn_def_args ::= (IDENT ',')* (IDENT ','?)

fn_body ::= '{' statement* '}' | statement | expr
```