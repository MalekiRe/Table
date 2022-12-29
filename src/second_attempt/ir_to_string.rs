use wasmtime::Val;
use crate::second_attempt::ir;
use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, Identifier, Value};

pub fn ir_to_str(file: ir::File) -> String {
    FileBuildingContext::from(file).create_string()
}
struct FileBuildingContext {
    fn_headers: Vec<CFnHeader>,
    fn_body: Vec<CFnBody>,
}
struct Scope {
    buffer: String,
    level: u32,
    num_local: u32,
    parent: Option<Box<Scope>>,
    stack: Vec<Identifier>,
    var_declare: String,
    var_decrement: String,
}
impl Scope {
    pub fn default() -> Self {
        Self {
            buffer: "".to_string(),
            level: 0,
            num_local: 0,
            parent: None,
            stack: vec![],
            var_declare: "".to_string(),
            var_decrement: "".to_string()
        }
    }
    pub fn add_local_var(&mut self, value: Value) {
        let generated_name = format!("_local_{}_{}", self.level, self.num_local);
        self.num_local += 1;
        self.var_declare.push_str(
            format!("Value* {} = {};", generated_name, 
                          match value {
                              Value::Number(number) => {
                                  format!("Number_new({})", number)
                              }
                              Value::String(string) => {
                                  format!("String_new(\"{}\")", string)
                              }
                              Value::Table(_) => unimplemented!(),
                          }
        ).as_str());
        self.var_decrement.push_str(
            format!("ref_dec({})", generated_name).as_str()
        );
        self.stack.push(generated_name);
    }
    pub fn push(self) -> Self {
        Scope {
            buffer: "".to_string(),
            level: self.level+1,
            num_local: 0,
            parent: Some(Box::new(self)),
            stack: vec![],
            var_declare: "".to_string(),
            var_decrement: "".to_string()
        }
    }
    pub fn pop(self) -> ScopePopType {
        match self.parent {
            None => {
                ScopePopType::Buffer(self.buffer)
            }
            Some(parent) => {
                let mut parent = *parent;
                parent.buffer += format!("{{\n{}\n{}\n{}\n}}", self.var_declare, self.buffer, self.var_decrement).as_str();
                ScopePopType::Scope(parent)
            }
        }
    }
}
enum ScopePopType {
    Scope(Scope),
    Buffer(String),
}
impl FileBuildingContext {
    pub fn from(file: ir::File) -> Self {
        let mut this = Self {
            fn_headers: vec![],
            fn_body: vec![],
        };
        this.private_from(file);
        this
    }
    fn private_from(&mut self, file: ir::File) {
        match file {
            File::Block(block) => {
                self.block(None, block);
            }
            File::None => {}
        }
    }
    fn block(&mut self, scope: Option<&mut Scope>, block: ir::Block) {
        let mut t = Scope::default();
        let mut scope = match scope {
            None => &mut t,
            Some(scope) => scope,
        };
        match block {
            Block::WithExp(statements, bexp) => {
                statements.into_iter().for_each(|statement| self.statement(&mut scope, *statement));
                self.expression(&mut scope, *bexp);
            }
            Block::WithoutExp(statements) => {
                statements.into_iter().for_each(|statement| self.statement(&mut scope, *statement));
            }
        }
    }
    fn statement(&mut self, scope: &mut Scope, statement: ir::Statement) {

    }
    fn expression(&mut self, scope: &mut Scope, expression: ir::Exp) {
        match expression {
            Exp::FnCall(_) => unimplemented!(),
            Exp::BinaryOperation(binary_operation) => {
                self.binary_operation(scope, binary_operation);
            },
            Exp::Value(value) => {
                self.value(scope, value);
            },
            Exp::Variable(_) => unimplemented!(),
        }
    }
    fn binary_operation(&mut self, scope: &mut Scope, binary_operation: BinaryOperation) {
        match binary_operation {
            BinaryOperation { left_hand_side, operator, right_hand_side } => {
                //TODO figure out lazy evaluation
                self.expression(scope, *left_hand_side);
                self.expression(scope, *right_hand_side);
                scope.buffer.push_str(
                  format!("Number_operation({}, {}, {})", scope.stack.pop().unwrap(),
                      scope.stack.pop().unwrap(),
                          match operator {
                              BinaryOperator::Add => "ADD",
                              BinaryOperator::Subtract => "SUBTRACT",
                              BinaryOperator::Multiply => "MULTIPLY",
                              BinaryOperator::Divide => "DIVIDE",
                              BinaryOperator::And => unimplemented!(),
                              BinaryOperator::Or => unimplemented!(),
                              BinaryOperator::EqualsEquals => unimplemented!()
                          },
                  ).as_str()
                );
            }
        }
    }
    fn value(&mut self, scope: &mut Scope, value: ir::Value) {
        scope.add_local_var(value);
    }
    pub fn gen_new_name(identifier: Identifier) -> Identifier {
        "_variable_".to_string()+identifier.as_str()
    }
    pub fn create_string(&self) -> String {
        String::default()
    }
}
type CFnHeader = String;
type CFnBody = String;