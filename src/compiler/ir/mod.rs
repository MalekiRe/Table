pub mod ir_bytecode_compiler;

use crate::misc::VecTuple1;

pub type IdentifierT = String;
pub type BExp = Box<Exp>;
pub type BStatement = Box<Statement>;

#[derive(Debug, PartialEq)]
pub enum Exp {
    ExpBlock(ExpBlock),
    LiteralValue(LiteralValue),
    FnCall(FnCall),
    TableOperation(TableOperation),
    Variable(IdentifierT),
    UnaryPrefixOperation(UnaryPrefixOperation),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug, PartialEq)]
pub enum TableOperation {
    /// this is like ```(a: my_thing, 1, "hi")[1]```
    TableIndexing{
        table: BExp,
        index: BExp
    },
    /// this is like ```some_table.my_func()``` where `my_func` takes in `self`
    TableMethodCalling{
        table: BExp,
        method: FnCall,
    },
    /// this is like ```some_table.some_val```
    TableFieldAccess {
        table: BExp,
        field: IdentifierT,
    },
    /// this is like ```some_table::foo()``` where `foo` doesn't take in `self`
    TableStaticFuncCalling{
        table: BExp,
        method: FnCall
    }
}
#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    Decimal(f64),
    Integer(isize),
    String(String),
    Table(TableLiteral),
    Boolean(bool),
}
/// `ExpBlock ::= { Statement* Exp }`
#[derive(Debug, PartialEq)]
pub struct ExpBlock(pub Vec<BStatement>, pub BExp);
/// `StatementBlock ::= { Statement+ }`
#[derive(Debug, PartialEq)]
pub struct StatementBlock(pub VecTuple1<BStatement>);

/// `OptionalStatementBlock ::= { Statement* }`
#[derive(Debug, PartialEq)]
pub enum OptionalStatementBlock {
    StatementBlock(StatementBlock),
    Empty,
}

/// `OptionalBlock ::= ExpBlock | OptionalStatementBlock`
/// `OptionalBlock ::= { Statement* Exp} | { Statement* }`
#[derive(Debug, PartialEq)]
pub enum OptionalBlock {
    ExpBlock(ExpBlock),
    OptionalStatementBlock(OptionalStatementBlock),
}
/// `Block ::= ExpBlock | StatementBlock`
/// `Block ::= { Statement* Exp } | { Statement+ }`
#[derive(Debug, PartialEq)]
pub enum Block {
    ExpBlock(ExpBlock),
    StatementBlock(StatementBlock),
}

/// `FnBody ::= OptionalBlock | Exp | Statement`
/// `FnBody ::= ('{' Statement* Exp? '}') | Exp | Statement`
#[derive(Debug, PartialEq)]
pub enum FnBody {
    OptionalBlock(OptionalBlock),
    Exp(BExp),
    Statement(BStatement)
}

#[derive(Debug, PartialEq)]
pub struct UnaryPostfixOperation {
    exp: BExp,
    op: UnaryPostfixOp,
}
#[derive(Debug, PartialEq)]
pub struct UnaryPrefixOperation {
    pub op: UnaryPrefixOp,
    pub exp: BExp,
}
#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub(crate) lhs: BExp,
    pub(crate) op: BinaryOp,
    pub(crate) rhs: BExp,
}
#[derive(Debug, PartialEq)]
pub enum UnaryPostfixOp {
    /// `foo++`
    Increment,
    /// `foo--`
    Decrement,
}
#[derive(Debug, PartialEq)]
pub enum UnaryPrefixOp {
    /// `!foo`
    Not,
}
#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Math(MathOp),
    Equality(EqualityOp),
}
#[derive(Debug, PartialEq)]
pub enum MathOp {
    /// `foo + bar`
    Add,
    /// `foo - bar`
    Subtract,
    /// `foo * bar`
    Multiply,
    /// `foo / bar`
    Divide,
    /// `foo % bar`
    Modulo,
    /// `foo += bar`
    AddEqual,
    /// `foo -= bar`
    MinusEqual,
    /// `foo *= bar`
    MultiplyEqual,
    /// `foo /= bar`
    DivideEqual,
    /// `foo %= bar`
    ModuloEqual,
}
#[derive(Debug, PartialEq)]
pub enum EqualityOp {
    /// `foo == bar`
    EqualsEquals,
    /// `foo != bar`
    EqualsNot,
    /// `foo >= bar`
    EqualsGreater,
    /// `foo <= bar`
    EqualsLess,
    /// `foo > bar`
    Greater,
    /// `foo < bar`
    Less,
    /// `foo & bar`
    And,
    /// `foo | bar`
    Or,
}
#[derive(Debug, PartialEq)]
pub enum Statement {
    FnDec(FnDec),
    FnImport(FnImport),
    LetStatement(LetStatement),
    ReassignmentStatement(ReassignmentStatement),
    ExpStatement(BExp),
    StatementBlock(OptionalStatementBlock),
    UnaryPostfixOperation(UnaryPostfixOperation),
}
#[derive(Debug, PartialEq)]
pub struct FnImport {
    pub(crate) identifier: IdentifierT,
    pub(crate) args: Vec<IdentifierT>,
}
#[derive(Debug, PartialEq)]
pub struct ReassignmentStatement {
    pub(crate) identifier: IdentifierT,
    pub(crate) lhs: BExp,
}
#[derive(Debug, PartialEq)]
pub struct FnDec {
    pub(crate) identifier: IdentifierT,
    pub(crate) args: Vec<IdentifierT>,
    pub(crate) closed_args: Vec<IdentifierT>,
    pub(crate) body: FnBody,
    pub(crate) exported: bool,
}
#[derive(Debug, PartialEq)]
pub struct FnCall {
    pub(crate) identifier: IdentifierT,
    pub(crate) args: Vec<BExp>,
}
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub(crate) identifier: IdentifierT,
    pub(crate) lhs: BExp,
}

pub type TableLiteral = Vec<TableKeyTemp>;
#[derive(Debug, PartialEq)]
pub struct TableKeyTemp {
    pub(crate) ident: Option<IdentifierT>,
    pub(crate) exp: BExp,
}
/// `File: JustStatements ::= Statement+`
/// `File: StatementExp ::= Statement* Exp`
/// `File: Empty ::= `
#[derive(Debug, PartialEq)]
pub enum File {
    /// `JustStatements ::= Statement+`
    JustStatements(VecTuple1<BStatement>),
    /// `StatementExp ::= Statement* Exp`
    StatementExp(Vec<BStatement>, BExp),
    /// `StatementExp ::= `
    Empty,
}