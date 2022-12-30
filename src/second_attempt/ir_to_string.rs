use replace_with::replace_with_or_abort;
use wasmtime::Val;
use crate::second_attempt::ir;
use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, FnDef, Identifier, Value};

// pub fn ir_to_str(file: ir::File) -> String {
//     FileBuildingContext::from(file).create_string()
// }
// struct FileBuildingContext {
//     fn_headers: Vec<CFnHeader>,
//     fn_body: Vec<CFnBody>,
// }
// struct Scope {
//     buffer: String,
//     level: u32,
//     num_local: u32,
//     parent: Option<Box<Scope>>,
//     stack: Vec<Identifier>,
//     var_declare: String,
//     var_decrement: String,
// }
// impl Scope {
//     pub fn default() -> Self {
//         Self {
//             buffer: "".to_string(),
//             level: 0,
//             num_local: 0,
//             parent: None,
//             stack: vec![],
//             var_declare: "".to_string(),
//             var_decrement: "".to_string()
//         }
//     }
//     pub fn add_local_var(&mut self, value: Value) {
//         let generated_name = format!("_local_{}_{}", self.level, self.num_local);
//         self.num_local += 1;
//         self.var_declare.push_str(
//             format!("Value* {} = {};", generated_name,
//                           match value {
//                               Value::Number(number) => {
//                                   format!("Number_new({})", number)
//                               }
//                               Value::String(string) => {
//                                   format!("String_new(\"{}\")", string)
//                               }
//                               Value::Table(_) => unimplemented!(),
//                           }
//         ).as_str());
//         self.var_decrement.push_str(
//             format!("ref_dec({})", generated_name).as_str()
//         );
//         self.stack.push(generated_name);
//     }
//     pub fn push(&mut self) {
//         replace_with_or_abort(self, |self_| {
//             Scope {
//                 buffer: "".to_string(),
//                 level: self_.level+1,
//                 num_local: 0,
//                 parent: Some(Box::new(self_)),
//                 stack: vec![],
//                 var_declare: "".to_string(),
//                 var_decrement: "".to_string()
//             }
//         })
//     }
//     pub fn pop(&mut self) -> ScopePopType {
//         match self.parent {
//             None => {
//                 ScopePopType::Buffer(self.buffer)
//             }
//             Some(parent) => {
//                 let mut parent = *parent;
//                 parent.buffer += format!("{{\n{}\n{}\n{}\n}}", self.var_declare, self.buffer, self.var_decrement).as_str();
//                 ScopePopType::Scope(parent)
//             }
//         }
//     }
// }
// enum ScopePopType {
//     Scope(Scope),
//     Buffer(String),
// }
// impl FileBuildingContext {
//     pub fn from(file: ir::File) -> Self {
//         let mut this = Self {
//             fn_headers: vec![],
//             fn_body: vec![],
//         };
//         this.private_from(file);
//         this
//     }
//     fn private_from(&mut self, file: ir::File) {
//         let main_fn_def = ir::FnDef {
//             identifier: "_main".to_string(),
//             args: vec![],
//             body: match file {
//                 File::Block(block) => {
//                     block
//                 }
//                 File::None => {
//                     Block::WithoutExp(vec![])
//                 }
//             }
//         };
//         fn_def(fn_def);
//     }
//     fn fn_def(&mut self, fn_def: ir::FnDef) {
//         let mut scope = Scope::default();
//         match fn_def {
//             FnDef { identifier, args, body } => {
//                 self.fn_headers.push(format!("Value* {}({});", identifier, Self::args_to_string(args)));
//                 self.block(&mut scope, body);
//                 match scope.pop() {
//                     ScopePopType::Scope(_) => panic!("somewhere we ended up pushing a scope we didn't pop!"),
//                     ScopePopType::Buffer(buffer) => {
//                         self.fn_body.push(format!("Value* {}({}){{{}}}", identifier, Self::args_to_string(args), buffer))
//                     }
//                 }
//             }
//         }
//     }
//     fn args_to_string(args: Vec<Identifier>) -> String {
//         let mut buffer = String::default();
//         let len = args.len();
//         for (i, arg) in args.into_iter().enumerate() {
//             buffer.push_str("Value* ");
//             buffer.push_str(arg.as_str());
//             if i != len {
//                 buffer.push_str(", ");
//             }
//         }
//         buffer
//     }
//     fn block(&mut self, scope: &mut Scope, block: ir::Block) {
//         scope.push();
//         match block {
//             Block::WithExp(statements, bexp) => {
//                 statements.into_iter().for_each(|statement| self.statement(scope, *statement));
//                 self.expression(scope, *bexp);
//             }
//             Block::WithoutExp(statements) => {
//                 statements.into_iter().for_each(|statement| self.statement(scope, *statement));
//             }
//         }
//     }
//     fn statement(&mut self, scope: &mut Scope, statement: ir::Statement) {
//
//     }
//     fn expression(&mut self, scope: &mut Scope, expression: ir::Exp) {
//         match expression {
//             Exp::FnCall(_) => unimplemented!(),
//             Exp::BinaryOperation(binary_operation) => {
//                 self.binary_operation(scope, binary_operation);
//             },
//             Exp::Value(value) => {
//                 self.value(scope, value);
//             },
//             Exp::Variable(_) => unimplemented!(),
//         }
//     }
//     fn binary_operation(&mut self, scope: &mut Scope, binary_operation: BinaryOperation) {
//         match binary_operation {
//             BinaryOperation { left_hand_side, operator, right_hand_side } => {
//                 //TODO figure out lazy evaluation
//                 self.expression(scope, *left_hand_side);
//                 self.expression(scope, *right_hand_side);
//                 scope.buffer.push_str(
//                   format!("Number_operation({}, {}, {})", scope.stack.pop().unwrap(),
//                       scope.stack.pop().unwrap(),
//                           match operator {
//                               BinaryOperator::Add => "ADD",
//                               BinaryOperator::Subtract => "SUBTRACT",
//                               BinaryOperator::Multiply => "MULTIPLY",
//                               BinaryOperator::Divide => "DIVIDE",
//                               BinaryOperator::And => unimplemented!(),
//                               BinaryOperator::Or => unimplemented!(),
//                               BinaryOperator::EqualsEquals => unimplemented!()
//                           },
//                   ).as_str()
//                 );
//             }
//         }
//     }
//     fn value(&mut self, scope: &mut Scope, value: ir::Value) {
//         scope.add_local_var(value);
//     }
//     pub fn gen_new_name(identifier: Identifier) -> Identifier {
//         "_variable_".to_string()+identifier.as_str()
//     }
//     pub fn create_string(&self) -> String {
//         String::default()
//     }
// }
// type CFnHeader = String;
// type CFnBody = String;