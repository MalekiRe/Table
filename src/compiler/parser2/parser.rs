use std::fmt::{Debug, Display, Formatter};
use chumsky::prelude::{any, empty, end, just, recursive};
use chumsky::{Parser, select, Span};
use crate::compiler::FileHolder;
use crate::compiler::parser2::lexer::{Control, lexer, Literal, Token};
use crate::compiler::parser2::error::{ErrorT, Pattern};
use crate::compiler::parser2::parser::ParseResult::{ParseErr, ParseOk};
use crate::compiler::parser2::parser::parsing_ir::{Block, Exp, ExpStatement, File, Statement};
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
    let block = block();
    let exp = exp(block.clone());
    let statement = statement(block.clone(), exp.clone());
    block.clone().then_ignore(end()).map(File::Block)
        .or(statement.clone().repeated().then(exp).map(|(statements, exp)| {
            File::StatementsExp(statements.into_iter().map(Box::new).collect(), Box::new(exp))
        }))
        .or(statement.clone().repeated().at_least(1).map(|s| {
            File::Statements(s.into_iter().map(Box::new).collect())
        }))
        .or(any().not().to(File::Empty))
}
pub fn block() -> impl TParser<Block> {
    recursive(|block| {
        let exp = exp(block.clone());
        let statement = statement(block.clone(), exp.clone());
        statement.map(Box::new).map(Block::Statement)
            .or(exp.map(Box::new).map(Block::Exp))
    })
}
pub fn exp(block: impl TParser<Block> + 'static) -> impl TParser<Exp> {
    recursive(|exp| {
        literal(exp.clone()).map(Exp::Literal)
    })
}
pub fn statement(block: impl TParser<Block> + 'static, exp: impl TParser<Exp> + 'static) -> impl TParser<Statement> {
    recursive(|statement| {
        exp_statement(exp.clone()).map(Statement::ExpStatement)
    })
}
pub fn exp_statement(exp: impl TParser<Exp>) -> impl TParser<ExpStatement> {
    exp.then_ignore(just(Token::Control(Control::Semicolon))).map(Box::new)
}
pub fn literal(exp: impl TParser<Exp>) -> impl TParser<parsing_ir::Literal> {
    let simple_literals = select!{
        Token::Literal(Literal::Char(c)) => parsing_ir::Literal::Char(c),
        Token::Literal(Literal::Boolean(b)) => parsing_ir::Literal::Boolean(b),
        Token::Literal(Literal::String(s)) => parsing_ir::Literal::String(s),
        Token::Literal(Literal::Number(lhs, rhs)) => parsing_ir::Literal::Number(format!("{}.{}", lhs, rhs).parse().unwrap())
    };
    let literal = simple_literals;
    literal.map_err(|e: ErrorT| e.expected(Pattern::Literal))
}
mod parsing_ir {
    use crate::compiler::parser2::lexer::Token;

    pub type IdentifierT = String;
    pub type BExp = Box<Exp>;
    pub type BStatement = Box<Statement>;

    pub type ExpStatement = BExp;
    #[derive(Clone)]
    pub enum OrInference<T> {
        Inference,
        Other(T),
    }
    #[derive(Clone)]
    pub struct FnCallArgs {
        args: Vec<BExp>,
    }
    #[derive(Clone)]
    pub struct FnDecArgs {
        args: Vec<IdentifierT>,
    }
    #[derive(Clone)]
    pub enum Block {
        Exp(BExp),
        Statement(BStatement),
    }
    #[derive(Clone)]
    pub enum File {
        Block(Block),
        StatementsExp(Vec<BStatement>, BExp),
        Statements(Vec<BStatement>), //at least 1
        Empty,
    }
    #[derive(Clone)]
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
    #[derive(Clone)]
    pub enum Variable {
        Identifier(IdentifierT),
        TableIndexing(TableIndexing),
        TableFieldAccess(TableFieldAccess),
    }
    #[derive(Clone)]
    pub struct LiteralCode(Vec<Token>);
    #[derive(Clone)]
    pub struct BinaryExp {
        lhs: BExp,
        op: BinaryOperator,
        rhs: BExp,
    }
    #[derive(Clone)]
    pub enum BinaryOperator {
        Add,
        Subtract,
        Multiply,
        Divide,
        Modulus,
        //todo the rest
    }
    #[derive(Clone)]
    pub struct FnCall {
        ident: IdentifierT,
        fn_call_args: FnCallArgs,
    }
    #[derive(Clone)]
    pub struct FnDec {
        dec_args: FnDecArgs,
        body: Block,
    }
    #[derive(Clone)]
    pub struct ExpBlock {
        statements: Vec<BStatement>,
        exp: BExp,
    }
    #[derive(Clone)]
    pub struct RangeCreation {
        lhs: BExp,
        rhs: BExp,
    }
    #[derive(Clone)]
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
    }
    #[derive(Clone)]
    pub enum Literal {
        Char(char),
        Number(f32),
        Boolean(bool),
        String(String),
        TableLiteral(TableLiteral),
    }
    #[derive(Clone)]
    pub struct TableLiteral {
        values: Vec<(Option<IdentifierT>, BExp)>
    }
    #[derive(Clone)]
    pub enum ControlFlowExp {
        MatchExp(MatchExp),
        LoopExp(LoopExp),
        ForExp(ForExp),
    }
    #[derive(Clone)]
    pub struct ForExp {
        key: IdentifierT,
        list: BExp,
        body: Block,
    }
    #[derive(Clone)]
    pub struct LoopExp {
        body: Block
    }
    #[derive(Clone)]
    pub struct MatchExp {
        matchee: Option<BExp>,
        body: Vec<MatchBody>,
    }
    #[derive(Clone)]
    pub struct MatchBody {
        head: MatchHead,
        arrow_type: ArrowType,
        body: Block,
    }
    #[derive(Clone)]
    pub enum ArrowType {
        Thick,
        Thin,
    }
    #[derive(Clone)]
    pub enum MatchHead {
        ExpWithoutSemicolon(OrInference<BExp>),
        ExpWithSemicolon(OrInference<BExp>),
        ExpWithConditional(OrInference<BExp>, BExp),
    }
    #[derive(Clone)]
    pub enum TableExp {
        TableIndexing(TableIndexing),
        TableMethodCall(TableMethodCall),
        TableStaticCall(TableStaticCall),
        TableFieldAccess(TableFieldAccess),
    }
    #[derive(Clone)]
    pub struct TableIndexing {
        table: BExp,
        index: BExp,
    }
    #[derive(Clone)]
    pub struct TableFieldAccess {
        table: BExp,
        field: IdentifierT,
    }
    #[derive(Clone)]
    pub struct TableMethodCall {
        table: BExp,
        fn_call_args: FnCallArgs,
    }
    #[derive(Clone)]
    pub struct TableStaticCall {
        table: BExp,
        fn_call_args: FnCallArgs,
    }
    #[derive(Clone)]
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

    #[derive(Clone)]
    pub enum ReturnStatement {
        NoReturnValue,
        Exp(BExp),
    }
    #[derive(Clone)]
    pub enum BreakStatement {
        NumberWithExp(u32, BExp),
        WithExp(BExp),
        Number(u32),
        Empty,
    }
    #[derive(Clone)]
    pub struct StatementBlock {
        statements: Vec<BStatement>,
    }
    #[derive(Clone)]
    pub enum ReassignStatement {
        SingleVarAssign(Variable, BExp),
        Table(TableReassign),
        UniqueIdentTable(UniqueIdentTableReassign),
    }
    #[derive(Clone)]
    pub struct TableReassign {
        table: Vec<LetIdentOrVar>,
        exp: BExp,
    }
    #[derive(Clone)]
    pub enum LetIdentOrVar {
        LetIdent(IdentifierT),
        Var(Variable),
    }
    #[derive(Clone)]
    pub struct UniqueIdentTableReassign {
        table: Vec<(IdentifierT, LetIdentOrVar)>,
        exp: BExp,
    }
    #[derive(Clone)]
    pub enum LetStatement {
        Uninitialized(IdentifierT),
        SingleAssign(IdentifierT, BExp),
        Table(TableAssign),
        UniqueIdentTable(UniqueIdentTableAssign),
    }
    #[derive(Clone)]
    pub struct TableAssign {
        table: Vec<IdentifierT>,
        exp: BExp,
    }
    #[derive(Clone)]
    pub struct UniqueIdentTableAssign {
        table: Vec<(IdentifierT, IdentifierT)>,
        exp: BExp,
    }
    #[derive(Clone)]
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