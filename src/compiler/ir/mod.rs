use crate::misc::VecTuple1;

pub type IdentifierT = String;
pub type BExp = Box<Exp>;
pub type BStatement = Box<Statement>;

pub enum Exp {
    ExpBlock(ExpBlock),
    LiteralValue(LiteralValue),
    FnCall(FnCall),
    TableOperation(TableOperation),
    Variable(IdentifierT),
    UnaryPostfixOperation(UnaryPostfixOperation),
    UnaryPrefixOperation(UnaryPrefixOperation),
    BinaryOperation(BinaryOperation),
}
pub enum Block {
    ExpBlock(ExpBlock),
    StatementBlock(StatementBlock),
}
pub enum MaybeEmptyBlock {
    ExpBlock(ExpBlock),
    StatementBlock(StatementBlock),
    Empty,
}
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

pub enum LiteralValue {
    Decimal(f64),
    Integer(isize),
    String(String),
    Table(TableLiteral),
    Boolean(bool),
}

pub struct ExpBlock(Vec<BStatement>, BExp);
pub struct StatementBlock(VecTuple1<BStatement>);
pub enum MaybeEmptyStatementBlock {
    StatementBlock(StatementBlock),
    Empty,
}
pub struct UnaryPostfixOperation {
    exp: BExp,
    op: UnaryPostfixOp,
}
pub struct UnaryPrefixOperation {
    op: UnaryPrefixOp,
    exp: BExp,
}
pub struct BinaryOperation {
    lhs: BExp,
    op: BinaryOp,
    rhs: BExp,
}
pub enum UnaryPostfixOp {
    /// `foo++`
    Increment,
    /// `foo--`
    Decrement,
}
pub enum UnaryPrefixOp {
    /// `!foo`
    Not,
}
pub enum BinaryOp {
    Math(MathOp),
    Equality(EqualityOp),
}
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
pub enum Statement {
    FnDec(FnDec),
    FnImport(FnImport),
    LetStatement(LetStatement),
    ExpStatement(BExp),
    StatementBlock(MaybeEmptyStatementBlock),
}
pub struct FnImport {
    identifier: IdentifierT,
    args: Vec<IdentifierT>,
}
pub struct FnDec {
    identifier: IdentifierT,
    args: Vec<IdentifierT>,
    body: Block,
    exported: bool,
}
pub struct FnCall {
    identifier: IdentifierT,
    args: Vec<BExp>,
}
pub struct LetStatement {
    identifier: IdentifierT,
    lhs: BExp,
}
pub type TableLiteral = Vec<(Option<IdentifierT>, BExp)>;
pub type File = MaybeEmptyBlock;