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
