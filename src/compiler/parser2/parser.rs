use std::fmt::{Debug, Display, Formatter};
use chumsky::prelude::{any, empty, end, filter_map, just, recursive};
use chumsky::{Parser, select, Span};
use chumsky::text::ident;
use crate::compiler::FileHolder;
use crate::compiler::ir::IdentifierT;
use crate::compiler::parser2::lexer::{Control, Keyword, lexer, Literal, Operator, Token};
use crate::compiler::parser2::error::{ErrorKind, ErrorT, Pattern};
use crate::compiler::parser2::lexer::Control::{Colon, Comma, LeftParen, LeftSquare, RightParen, RightSquare};
use crate::compiler::parser2::parser::ParseResult::{ParseErr, ParseOk};
use crate::compiler::parser2::parser::parsing_ir::{ArrowType, BinaryExp, Block, Exp, ExpBlock, ExpStatement, File, FnCall, FnCallArgs, FnDec, FnDecArgs, LetStatement, RangeCreation, ReassignStatement, Statement, TableAssign, TableExp, TableFieldAccess, TableIndexing, TableLiteral, TableLiteralEntry, TableMethodCall, TableStaticCall, UniqueIdentTableAssign, Variable};
use crate::compiler::parser::error;
use crate::compiler::parser::span::TSpan;

pub trait TParser<T> = chumsky::Parser<Token, T, Error =ErrorT> + Clone;

#[derive(Debug, Clone)]
pub struct ParsingError {
    errors: Vec<ErrorT>,
    file_holder: String,
}

impl ParsingError {
    pub fn from(src: &str, errors: Vec<ErrorT>) -> Self {
        Self {
            errors,
            file_holder: src.to_string()
        }
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = Vec::new();
        let mut file_holder = FileHolder::from(self.file_holder.clone());
        let errors = self.errors.clone();
        for error in errors {
            error.write(&mut file_holder, &mut str);
        }
        write!(f, "{:#?}", String::from_utf8(str))
    }
}

impl ParsingError {
    pub fn write_error(&self) {
        let mut file_holder = FileHolder::from(self.file_holder.clone());
        let errors = self.errors.clone();
        for error in errors {
            error.write(&mut file_holder, std::io::stderr());
        }
    }
    pub fn panic_write(&self) -> ! {
        self.write_error();
        panic!();
    }
}

pub enum ParseResult<T> {
    ParseErr(ParsingError),
    ParseOk(T)
}
impl<T> ParseResult<T> {
    pub(crate) fn unwrap(self) -> T {
        match self {
            ParseResult::ParseErr(err) => {
                err.write_error();
                panic!("parse error");
            }
            ParseResult::ParseOk(val) => val,
        }
    }
}


pub fn parse(src: &str) -> ParseResult<File>{
    let len = src.chars().count();
    let my_span = Span::new(0, len..len);
    let (tokens, mut errors) = lexer()
        .parse_recovery(chumsky::Stream::from_iter(
           my_span,
            src
                .chars()
                .enumerate()
                .map(|(i, c)| (c, TSpan::new(0, i..i +1)))
        ));
    let tokens = match tokens {
        None => return ParseErr(ParsingError::from(src, errors)),
        Some(tokens) => tokens,
    };
    let (file, mut parser_errors) = file().parse_recovery(chumsky::Stream::from_iter(my_span, tokens.into_iter()));
    errors.append(&mut parser_errors);
    match file {
        None => ParseErr(ParsingError::from(src, errors)),
        Some(file) => ParseOk(file),
    }
}
pub fn file() -> impl TParser<parsing_ir::File> {
    let exp = exp();
    let statement = statement(exp.clone());
    statement.clone().repeated().then(exp).then_ignore(end()).map(|(statements, exp)| {
        File::StatementsExp(statements.into_iter().map(Box::new).collect(), Box::new(exp))
    })
    .or(statement.clone().repeated().then_ignore(end()).map(|s| {
        File::Statements(s.into_iter().map(Box::new).collect())
    }))
        //.or(any().not().then_ignore(end()).to(File::Empty))
}
pub fn block(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<Block> {
    exp.map(|e| Block::Exp(Box::new(e)))
        .or(statement.map(|s| Block::Statement(Box::new(s))))
}
pub fn exp() -> impl TParser<Exp> {
    recursive(|exp| {
        let statement = statement(exp.clone());
        let block = block(exp.clone(), statement.clone());

        let literal = literal(exp.clone()).map(Exp::Literal);
        let exp_block = exp_block(statement.clone(), exp.clone()).map(Exp::ExpBlock);
        let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
        let range_creation = range_creation(exp.clone()).map(Exp::RangeCreation);
        let fn_dec = fn_dec(block.clone()).map(Exp::FnDec);
        let table_exp = table_exp(exp.clone(), statement.clone()).map(Exp::TableExp);

        range_creation.or(exp_block).or(fn_dec).or(table_exp).or(fn_call).or(literal)
    })
}
pub fn identifier() -> impl TParser<IdentifierT> {
    let ident = filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(ErrorT::new(ErrorKind::Unexpected(Pattern::TermIdent), span)),
    });
    ident
}
//table_exp
//{
pub fn table_exp(exp: impl TParser<Exp> + 'static, statement: impl TParser<Statement> + 'static) -> impl TParser<TableExp> {
    recursive(|table_exp| {
        let table = _table(exp.clone(), statement.clone());

        let table_indexing = table_indexing(table.clone(), exp.clone(), statement.clone()).map(TableExp::TableIndexing);
        let table_field_access = table_field_access(table.clone()).map(TableExp::TableFieldAccess);
        let table_method_call = table_method_call(table.clone(), exp.clone()).map(TableExp::TableMethodCall);
        let table_static_call = table_static_call(table.clone(), exp.clone()).map(TableExp::TableStaticCall);

        table_indexing.or(table_method_call).or(table_static_call).or(table_field_access)
    })
}
/// for internal use
pub fn _table(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<Exp> {

    let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
    //let binary_exp; //TODO
    let exp_block = exp_block(statement, exp.clone()).map(Exp::ExpBlock);
    //let control_flow_exp; //TODO
    let table_literal = table_literal(exp.clone()).map(parsing_ir::Literal::TableLiteral).map(Exp::Literal);

    let table = table_literal.or(fn_call.or(exp_block));
    table
}
pub fn table_indexing(table: impl TParser<Exp>, exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<TableIndexing> {

    let fn_call = fn_call(exp.clone()).map(Exp::FnCall);
    //let binary_exp; //TODO
    //let control_flow_exp; //TODO
    let range_creation = range_creation(exp.clone()).map(Exp::RangeCreation);
    let exp_block = exp_block(statement, exp.clone()).map(Exp::ExpBlock);
    let simple_literal = simple_literal().map(Exp::Literal);

    let index = range_creation.or(fn_call.or(exp_block.or(simple_literal)));
    table.then(index.delimited_by(just(Token::Control(LeftSquare)), just(Token::Control(RightSquare))))
        .map(|(table, index)| {
            TableIndexing {
                table: Box::new(table),
                index: Box::new(index)
            }
        })
}
pub fn table_field_access(table: impl TParser<Exp>) -> impl TParser<TableFieldAccess> {
    table.then_ignore(just(Token::Control(Control::Dot))).then(identifier())
        .map(|(table, field)| {
            TableFieldAccess {
                table: Box::new(table),
                field
            }
        })
}
pub fn table_method_call(table: impl TParser<Exp>, exp: impl TParser<Exp>) -> impl TParser<TableMethodCall> {
    table.then_ignore(just(Token::Control(Control::Dot)))
        .then(fn_call(exp))
        .map(|(table, fn_call)| {
            TableMethodCall {
                table: Box::new(table),
                fn_call
            }
        })
}
pub fn table_static_call(table: impl TParser<Exp>, exp: impl TParser<Exp>) -> impl TParser<TableStaticCall> {
    table.then_ignore(just(Token::Control(Control::Colon)).repeated().exactly(2))
        .then(fn_call(exp))
        .map(|(table, fn_call)| {
            TableStaticCall {
                table: Box::new(table),
                fn_call
            }
        })
}


//}
pub fn fn_call_args(exp: impl TParser<Exp>) -> impl TParser<FnCallArgs> {
    let with_args = exp.separated_by(just(Token::Control(Control::Comma))).allow_trailing()
        .delimited_by(just(Token::Control(LeftParen)), just(Token::Control(RightParen)))
        .map(|args| {
            FnCallArgs {
                args: args.into_iter().map(Box::new).collect()
            }
        });
    with_args
}
pub fn fn_call(exp: impl TParser<Exp>) -> impl TParser<FnCall> {
    identifier().then(fn_call_args(exp))
        .map(|(ident, args)| {
            FnCall {
                ident,
                fn_call_args: args
            }
        })
}
pub fn fn_dec_args() -> impl TParser<FnDecArgs> {
    let with_args = identifier().separated_by(just(Token::Control(Comma))).allow_trailing()
        .delimited_by(just(Token::Control(LeftParen)), just(Token::Control(RightParen)))
        .map(|args| {
            FnDecArgs {
                args
            }
        });
    with_args
}
pub fn fn_dec(block: impl TParser<Block>) -> impl TParser<FnDec> {
    fn_dec_args().then_ignore(just(Token::Operator(Operator::SkinnyArrow))).then(block)
        .map(|(dec_args,  block)| {
            FnDec {
                dec_args,
                body: block
            }
        })
}
pub fn range_creation(exp: impl TParser<Exp>) -> impl TParser<RangeCreation> {
    let literal = literal(exp.clone()).map(Exp::Literal);
    let fn_call = fn_call(exp.clone()).map(Exp::FnCall);

    let exp = literal.or(fn_call);
    exp.clone().then_ignore(just(Token::Operator(Operator::Range))).then(exp)
        .map(|(lhs, rhs)| {
            RangeCreation {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs)
            }
        })
}
pub fn exp_block(statement: impl TParser<Statement>, exp: impl TParser<Exp>) -> impl TParser<ExpBlock> {
    statement.repeated().then(exp).delimited_by(just(Token::Control(Control::LeftCurly)), just(Token::Control(Control::RightCurly)))
        .map(|(statements, exp)| {
            ExpBlock {
                statements: statements.into_iter().map(Box::new).collect(),
                exp: Box::new(exp)
            }
        })
}
//statement
//{
pub fn statement(exp: impl TParser<Exp> + 'static) -> impl TParser<Statement> {
    recursive(|statement| {
        let exp_statement = exp_statement(exp.clone()).map(Statement::ExpStatement);
        let let_statement = let_statement(exp.clone()).map(Statement::LetStatement);
        let reassign_statement = reassign_statement(exp.clone(), statement.clone()).map(Statement::ReassignStatement);
        let_statement.or(reassign_statement).or(exp_statement)
    })
}
pub fn exp_statement(exp: impl TParser<Exp>) -> impl TParser<ExpStatement> {
    exp.then_ignore(just(Token::Control(Control::Semicolon))).map(Box::new)
}
pub fn let_statement(exp: impl TParser<Exp>) -> impl TParser<LetStatement> {
    let uninit = r#let().ignore_then(identifier()).then_ignore(semicolon())
        .map(|identifier| {
            LetStatement::Uninitialized(identifier)
        });
    let simple = r#let().ignore_then(identifier()).then_ignore(equals())
        .then(exp.clone()).then_ignore(semicolon())
        .map(|(identifier, lhs)| {
            LetStatement::SingleAssign(identifier, Box::new(lhs))
        });

    let table_syntax_without_ident = r#let().ignore_then(
        identifier().separated_by(comma()).delimited_by(left_paren(), right_paren())
    ).then_ignore(equals()).then(exp.clone()).then_ignore(semicolon())
        .map(|(table, exp)| {
            LetStatement::Table(TableAssign {
                table,
                exp: Box::new(exp)
            })
        });
    let table_syntax_with_ident = r#let().ignore_then(
        identifier().then_ignore(colon()).then(identifier())
            .separated_by(comma()).delimited_by(left_paren(), right_paren())
    ).then_ignore(equals()).then(exp).then_ignore(semicolon())
        .map(|(table, exp)| {
            LetStatement::UniqueIdentTable(UniqueIdentTableAssign {
                table,
                exp: Box::new(exp)
            })
        });
    uninit.or(simple).or(table_syntax_with_ident).or(table_syntax_without_ident)
}
pub fn reassign_statement(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<ReassignStatement> {
    let single = variable(exp.clone(), statement.clone()).then_ignore(equals()).then(exp.clone()).then_ignore(semicolon())
        .map(|(var, exp)| {
            ReassignStatement::SingleVarAssign(var, Box::new(exp))
        });

    single
}
//}
pub fn variable(exp: impl TParser<Exp>, statement: impl TParser<Statement>) -> impl TParser<Variable> {
    let table = _table(exp.clone(), statement.clone());
    let table_indexing = table_indexing(table.clone(), exp.clone(), statement.clone())
        .map(Variable::TableIndexing);
    let table_field_access = table_field_access(table.clone())
        .map(Variable::TableFieldAccess);
    let identifier = identifier().map(Variable::Identifier);
    table_field_access.or(table_indexing).or(identifier)
}
pub fn colon() -> impl TParser<Token> {
    just(Token::Control(Control::Colon))
}
pub fn equals() -> impl TParser<Token> {
    just(Token::Operator(Operator::Equals))
}
pub fn left_paren() -> impl TParser<Token> {
    just(Token::Control(Control::LeftParen))
}
pub fn right_paren() -> impl TParser<Token> {
    just(Token::Control(Control::RightParen))
}
pub fn comma() -> impl TParser<Token> {
    just(Token::Control(Control::Comma))
}
pub fn r#let() -> impl TParser<Token> {
    just(Token::Keyword(Keyword::Let))
}
pub fn semicolon() -> impl TParser<Token> {
    just(Token::Control(Control::Semicolon))
}
pub fn simple_literal() -> impl TParser<parsing_ir::Literal> {
    select!{
        Token::Literal(Literal::Char(c)) => parsing_ir::Literal::Char(c),
        Token::Literal(Literal::Boolean(b)) => parsing_ir::Literal::Boolean(b),
        Token::Literal(Literal::String(s)) => parsing_ir::Literal::String(s),
        Token::Literal(Literal::Number(lhs, rhs)) => parsing_ir::Literal::Number(format!("{}.{}", lhs, rhs).parse().unwrap())
    }
}
pub fn literal(exp: impl TParser<Exp>) -> impl TParser<parsing_ir::Literal> {
    let simple_literal = simple_literal();
    let table_literal = table_literal(exp).map(parsing_ir::Literal::TableLiteral);
    simple_literal.or(table_literal).map_err(|e: ErrorT| e.expected(Pattern::Literal))
}
pub fn table_literal(exp: impl TParser<Exp>) -> impl TParser<parsing_ir::TableLiteral> {
    let entry = identifier().then_ignore(just(Token::Control(Colon))).or_not().then(exp)
        .map(|(ident, exp)| {
            TableLiteralEntry {
                ident,
                val: Box::new(exp)
            }
        });
    entry.separated_by(just(Token::Control(Comma))).allow_trailing().delimited_by(just(Token::Control(LeftParen)), just(Token::Control(RightParen)))
        .map(|entries| {
            TableLiteral {
                values: entries
            }
        })
}
mod parsing_ir {
    use crate::compiler::parser2::lexer::Token;

    pub type IdentifierT = String;
    pub type BExp = Box<Exp>;
    pub type BStatement = Box<Statement>;

    pub type ExpStatement = BExp;
    #[derive(Clone, Debug)]
    pub enum OrInference<T> {
        Inference,
        Other(T),
    }
    #[derive(Clone, Debug)]
    pub struct FnCallArgs {
        pub(crate) args: Vec<BExp>,
    }
    #[derive(Clone, Debug)]
    pub struct FnDecArgs {
        pub(crate) args: Vec<IdentifierT>,
    }
    #[derive(Clone, Debug)]
    pub enum Block {
        Exp(BExp),
        Statement(BStatement),
    }
    #[derive(Clone, Debug)]
    pub enum File {
        StatementsExp(Vec<BStatement>, BExp),
        Statements(Vec<BStatement>),
        //Empty,
    }
    #[derive(Clone, Debug)]
    pub enum MacroCall {
        Standard {
            ident: IdentifierT,
            fn_call_args: FnCallArgs,
        },
        TableMethod {
            table: BExp,
            fn_call_args: FnCallArgs,
        }
    }
    #[derive(Clone, Debug)]
    pub enum Variable {
        Identifier(IdentifierT),
        TableIndexing(TableIndexing),
        TableFieldAccess(TableFieldAccess),
    }
    #[derive(Clone, Debug)]
    pub struct LiteralCode(Vec<Token>);
    #[derive(Clone, Debug)]
    pub struct BinaryExp {
        lhs: BExp,
        op: BinaryOperator,
        rhs: BExp,
    }
    #[derive(Clone, Debug)]
    pub enum BinaryOperator {
        Add,
        Subtract,
        Multiply,
        Divide,
        Modulus,
        //todo the rest
    }
    #[derive(Clone, Debug)]
    pub struct FnCall {
        pub(crate) ident: IdentifierT,
        pub(crate) fn_call_args: FnCallArgs,
    }
    #[derive(Clone, Debug)]
    pub struct FnDec {
        pub(crate) dec_args: FnDecArgs,
        pub(crate) body: Block,
    }
    #[derive(Clone, Debug)]
    pub struct ExpBlock {
        pub(crate) statements: Vec<BStatement>,
        pub(crate) exp: BExp,
    }
    #[derive(Clone, Debug)]
    pub struct RangeCreation {
        pub(crate) lhs: BExp,
        pub(crate) rhs: BExp,
    }
    #[derive(Clone, Debug)]
    pub enum Exp {
        BinaryExp(BinaryExp),
        ExpBlock(ExpBlock),
        FnCall(FnCall),
        FnDec(FnDec),
        TableExp(TableExp),
        ControlFlowExp(ControlFlowExp),
        RangeCreation(RangeCreation),
        MacroCall(MacroCall),
        LiteralCode(LiteralCode),
        Literal(Literal),
        VariableIdentifier(IdentifierT),
    }
    #[derive(Clone, Debug)]
    pub enum Literal {
        Char(char),
        Number(f32),
        Boolean(bool),
        String(String),
        TableLiteral(TableLiteral),
    }
    #[derive(Clone, Debug)]
    pub struct TableLiteral {
        pub(crate) values: Vec<TableLiteralEntry>
    }
    #[derive(Clone, Debug)]
    pub struct TableLiteralEntry {
        pub(crate) ident: Option<IdentifierT>,
        pub(crate) val: BExp
    }
    #[derive(Clone, Debug)]
    pub enum ControlFlowExp {
        MatchExp(MatchExp),
        LoopExp(LoopExp),
        ForExp(ForExp),
    }
    #[derive(Clone, Debug)]
    pub struct ForExp {
        key: IdentifierT,
        list: BExp,
        body: Block,
    }
    #[derive(Clone, Debug)]
    pub struct LoopExp {
        body: Block
    }
    #[derive(Clone, Debug)]
    pub struct MatchExp {
        matchee: Option<BExp>,
        body: Vec<MatchBody>,
    }
    #[derive(Clone, Debug)]
    pub struct MatchBody {
        head: MatchHead,
        arrow_type: ArrowType,
        body: Block,
    }
    #[derive(Clone, Debug)]
    pub enum ArrowType {
        Thick,
        Thin,
    }
    #[derive(Clone, Debug)]
    pub enum MatchHead {
        ExpWithoutSemicolon(OrInference<BExp>),
        ExpWithSemicolon(OrInference<BExp>),
        ExpWithConditional(OrInference<BExp>, BExp),
    }
    #[derive(Clone, Debug)]
    pub enum TableExp {
        TableIndexing(TableIndexing),
        TableMethodCall(TableMethodCall),
        TableStaticCall(TableStaticCall),
        TableFieldAccess(TableFieldAccess),
    }
    #[derive(Clone, Debug)]
    pub struct TableIndexing {
        pub(crate) table: BExp,
        pub(crate) index: BExp,
    }
    #[derive(Clone, Debug)]
    pub struct TableFieldAccess {
        pub(crate) table: BExp,
        pub(crate) field: IdentifierT,
    }
    #[derive(Clone, Debug)]
    pub struct TableMethodCall {
        pub(crate) table: BExp,
        pub(crate) fn_call: FnCall,
    }
    #[derive(Clone, Debug)]
    pub struct TableStaticCall {
        pub(crate) table: BExp,
        pub(crate) fn_call: FnCall,
    }
    #[derive(Clone, Debug)]
    pub enum Statement {
        StatementBlock(StatementBlock),
        LetStatement(LetStatement),
        ReassignStatement(ReassignStatement),
        BreakStatement(BreakStatement),
        IfStatement(IfStatement),
        ReturnStatement(ReturnStatement),
        ExpStatement(BExp),
        MacroCall(MacroCall),
    }

    #[derive(Clone, Debug)]
    pub enum ReturnStatement {
        NoReturnValue,
        Exp(BExp),
    }
    #[derive(Clone, Debug)]
    pub enum BreakStatement {
        NumberWithExp(u32, BExp),
        WithExp(BExp),
        Number(u32),
        Empty,
    }
    #[derive(Clone, Debug)]
    pub struct StatementBlock {
        statements: Vec<BStatement>,
    }
    #[derive(Clone, Debug)]
    pub enum ReassignStatement {
        SingleVarAssign(Variable, BExp),
        Table(TableReassign),
        UniqueIdentTable(UniqueIdentTableReassign),
    }
    #[derive(Clone, Debug)]
    pub struct TableReassign {
        table: Vec<LetIdentOrVar>,
        exp: BExp,
    }
    #[derive(Clone, Debug)]
    pub enum LetIdentOrVar {
        LetIdent(IdentifierT),
        Var(Variable),
    }
    #[derive(Clone, Debug)]
    pub struct UniqueIdentTableReassign {
        table: Vec<(IdentifierT, LetIdentOrVar)>,
        exp: BExp,
    }
    #[derive(Clone, Debug)]
    pub enum LetStatement {
        Uninitialized(IdentifierT),
        SingleAssign(IdentifierT, BExp),
        Table(TableAssign),
        UniqueIdentTable(UniqueIdentTableAssign),
    }
    #[derive(Clone, Debug)]
    pub struct TableAssign {
        pub(crate) table: Vec<IdentifierT>,
        pub(crate) exp: BExp,
    }
    #[derive(Clone, Debug)]
    pub struct UniqueIdentTableAssign {
        pub(crate) table: Vec<(IdentifierT, IdentifierT)>,
        pub(crate) exp: BExp,
    }
    #[derive(Clone, Debug)]
    pub struct IfStatement {
        conditional: BExp,
        body: Block,
    }
}
/*
The Grammar

block ::= exp | statement
file ::= block | statements* exp | statements+ | EMPTY

macro_call ::=
           IDENT '!' '(' fn_call_args  ') |
           exp '.' '!' '(' fn_call_args ')'

variable   ::= IDENT | table_indexing | table_field_access

literal_code ::= '#{' ANYTHING '}#'
binary_exp ::= exp BINARY_OP exp
fn_call    ::= IDENT '(' call_args ')'
fn_dec     ::= '(' dec_args ')' '->' block
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

    match_head ::= (exp | '_') | (exp | '_') ';' | (exp | '_') ';' exp

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