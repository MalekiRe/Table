pub mod ir_bytecode_compiler;

use crate::misc::VecTuple1;

pub type IdentifierT = String;
pub type BExp = Box<Exp>;
pub type BStatement = Box<Statement>;

#[derive(Debug)]
pub enum Exp {
    ExpBlock(ExpBlock),
    LiteralValue(LiteralValue),
    FnCall(FnCall),
    TableOperation(TableOperation),
    Variable(IdentifierT),
    UnaryPrefixOperation(UnaryPrefixOperation),
    BinaryOperation(BinaryOperation),
}
#[derive(Debug)]
pub enum Block {
    ExpBlock(ExpBlock),
    StatementBlock(StatementBlock),
}
#[derive(Debug)]
pub enum MaybeEmptyBlock {
    ExpBlock(ExpBlock),
    StatementBlock(StatementBlock),
    Empty,
}
#[derive(Debug)]
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
    TableArgAccess{
        table: BExp,
        arg: IdentifierT,
    },
    /// this is like ```some_table::foo()``` where `foo` doesn't take in `self`
    TableStaticFuncCalling{
        table: BExp,
        method: FnCall
    }
}
#[derive(Debug)]
pub enum LiteralValue {
    Decimal(f64),
    Integer(isize),
    String(String),
    Table(TableLiteral),
    Boolean(bool),
}
#[derive(Debug)]
pub struct ExpBlock(Vec<BStatement>, BExp);
#[derive(Debug)]
pub struct StatementBlock(VecTuple1<BStatement>);
#[derive(Debug)]
pub enum MaybeEmptyStatementBlock {
    StatementBlock(StatementBlock),
    Empty,
}
#[derive(Debug)]
pub struct UnaryPostfixOperation {
    exp: BExp,
    op: UnaryPostfixOp,
}
#[derive(Debug)]
pub struct UnaryPrefixOperation {
    op: UnaryPrefixOp,
    exp: BExp,
}
#[derive(Debug)]
pub struct BinaryOperation {
    lhs: BExp,
    op: BinaryOp,
    rhs: BExp,
}
#[derive(Debug)]
pub enum UnaryPostfixOp {
    /// `foo++`
    Increment,
    /// `foo--`
    Decrement,
}
#[derive(Debug)]
pub enum UnaryPrefixOp {
    /// `!foo`
    Not,
}
#[derive(Debug)]
pub enum BinaryOp {
    Math(MathOp),
    Equality(EqualityOp),
}
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Statement {
    FnDec(FnDec),
    FnImport(FnImport),
    LetStatement(LetStatement),
    ExpStatement(BExp),
    StatementBlock(MaybeEmptyStatementBlock),
    UnaryPostfixOperation(UnaryPostfixOperation),
}
#[derive(Debug)]
pub struct FnImport {
    identifier: IdentifierT,
    args: Vec<IdentifierT>,
}
#[derive(Debug)]
pub struct FnDec {
    identifier: IdentifierT,
    args: Vec<IdentifierT>,
    body: Block,
    exported: bool,
}
#[derive(Debug)]
pub struct FnCall {
    pub(crate) identifier: IdentifierT,
    pub(crate) args: Vec<BExp>,
}
#[derive(Debug)]
pub struct LetStatement {
    identifier: IdentifierT,
    lhs: BExp,
}

pub type TableLiteral = Vec<TableKeyTemp>;
#[derive(Debug)]
pub struct TableKeyTemp {
    pub(crate) ident: Option<IdentifierT>,
    pub(crate) exp: BExp,
}
pub type File = MaybeEmptyBlock;