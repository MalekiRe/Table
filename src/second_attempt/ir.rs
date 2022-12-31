use crate::second_attempt::lexer;
pub type BExp = Box<Exp>;
pub type BStatement = Box<Statement>;
pub type Identifier = String;

#[derive(Debug)]
pub enum Exp {
    FnCall(FnCall),
    BinaryOperation(BinaryOperation),
    Value(Value),
    Variable(Identifier),
    Block(Vec<BStatement>, BExp),
}
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    EqualsEquals,
}
// TODO:: Remember to lazily evaluate binary operations
#[derive(Debug)]
pub struct BinaryOperation {
    pub left_hand_side: BExp,
    pub operator: BinaryOperator,
    pub right_hand_side: BExp,
}
#[derive(Debug)]
pub struct FnCall {
    pub(crate) identifier: Identifier,
    pub(crate) args: Vec<BExp>,
}
#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Table(Table),
}
#[derive(Debug)]
pub enum TableKey {
    HasString(String),
    NoString
}
#[derive(Debug)]
pub struct Table(pub Vec<(TableKey, BExp)>);
#[derive(Debug)]
pub enum File {
    Block(Block),
    None,
}
#[derive(Debug)]
pub enum Statement {
    FnDef(FnDef),
    LetStatement(LetStatement),
    ExpStatement(BExp),
    Block(Vec<BStatement>),
}
#[derive(Debug)]
pub struct LetStatement {
    pub identifier: Identifier,
    pub exp: BExp,
}
#[derive(Debug)]
pub struct NormalFnDef {
    pub(crate) identifier: Identifier,
    pub(crate) args: Vec<Identifier>,
    pub(crate) body: Block,
    pub(crate) closure_idents: Vec<Identifier>,
    pub(crate) exported: bool,
}
#[derive(Debug)]
pub struct ImportedFnDef {
    pub(crate) identifier: Identifier,
    pub(crate) args: Vec<Identifier>,
}
#[derive(Debug)]
pub enum FnDef {
    FnDef(NormalFnDef),
    Imported(ImportedFnDef)
}

// some sort of scoped section
#[derive(Debug)]
pub enum Block {
    WithExp(Vec<BStatement>, BExp),
    WithoutExp(Vec<BStatement>),
}