use crate::second_attempt::lexer;
pub type BExp = Box<Exp>;
pub type BStatement = Box<Statement>;
pub type Identifier = String;

pub enum Exp {
    FnCall(FnCall),
    BinaryOperation(BinaryOperation),
    Value(Value),
    Variable(Identifier),
}
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
pub struct BinaryOperation {
    pub left_hand_side: BExp,
    pub operator: BinaryOperator,
    pub right_hand_side: BExp,
}
pub struct FnCall {
    pub(crate) identifier: Identifier,
    pub(crate) args: Vec<BExp>,
}
pub enum Value {
    Number(f64),
    String(String),
    Table(Table),
}
pub enum TableKey {
    HasString(String),
    NoString
}
pub struct Table(pub Vec<(TableKey, BExp)>);
pub enum File {
    Block(Block),
    None,
}
pub enum Statement {
    FnDef(FnDef),
    LetStatement(LetStatement),
    ExpStatement(BExp),
    Block(Block),
}
pub struct LetStatement {
    pub identifier: Identifier,
    pub exp: BExp,
}
pub struct FnDef {
    pub(crate) identifier: Identifier,
    pub(crate) args: Vec<Identifier>,
    pub(crate) body: Block,
    pub(crate) closure_idents: Vec<Identifier>,
}
// some sort of scoped section
pub enum Block {
    WithExp(Vec<BStatement>, BExp),
    WithoutExp(Vec<BStatement>),
}