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
    identifier: Identifier,
    args: Vec<BExp>,
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
    LetStatement(BExp),
    ExpStatement(BExp),
    Block(Block),
}
pub struct LetStatement {
    identifier: Identifier,
    exp: BExp,
}
pub struct FnDef {
    identifier: Identifier,
    args: Vec<Identifier>,
    body: Block,
}
// some sort of scoped section
pub enum Block {
    WithExp(Vec<BStatement>, BExp),
    WithoutExp(Vec<BStatement>),
}