use crate::second_attempt::ir;
use crate::second_attempt::ir::{BinaryOperation, BinaryOperator, Block, Exp, File, FnDef, Statement, Value};

// pub type Identifier = String;
// pub type Buffer = String;
// pub struct Scope {
//     num_local: u32,
//     level: u32,
//     buffer: Buffer,
//     var_declare: Buffer,
//     var_decrement: Buffer,
//     var_increment: Buffer,
//     return_line: Option<ReturnLine>,
//     parent: Option<Box<Scope>>,
// }
// impl Scope {
//     pub fn pop(self) -> (Option<Box<Self>>, Buffer) {
//         let buffer = self.combine_buffers();
//         match self {
//             Scope { parent, .. } => {
//                 (parent, buffer)
//             }
//         }
//     }
//     pub fn default() -> Self {
//         Self {
//             num_local: 0,
//             level: 0,
//             buffer: "".to_string(),
//             var_declare: "".to_string(),
//             var_decrement: "".to_string(),
//             var_increment: "".to_string(),
//             return_line: None,
//             parent: None
//         }
//     }
//     pub fn combine_buffers(&self) -> Buffer {
//         format!("{}{}{}{}{}", self.var_declare, self.buffer, self.var_increment, self.var_decrement, match &self.return_line {
//             None => {String::default()}
//             Some(return_line) => {
//                 format!("return {};", return_line.identifier)
//             }
//         })
//     }
// }
// pub struct ScopeHolder {
//     inner: Option<Scope>,
//     stack: Vec<Identifier>,
// }
// impl ScopeHolder {
//     pub fn new() -> Self {
//         Self {
//             inner: Some(Scope::default()),
//             stack: vec![]
//         }
//     }
//     pub fn push_scope(&mut self) {
//         let this = self.inner.take().unwrap();
//         self.inner.replace(Scope {
//             num_local: 0,
//             level: this.level+1,
//             buffer: "".to_string(),
//             var_declare: "".to_string(),
//             var_decrement: "".to_string(),
//             var_increment: "".to_string(),
//             return_line: None,
//             parent: Some(Box::new(this))
//         });
//     }
//     /// returns a buffer if this is the last scope
//     pub fn pop_scope(&mut self) -> Option<Buffer> {
//         let (parent, buffer) = self.inner.take().unwrap().pop();
//         match parent {
//             None => {
//                 Some(buffer)
//             }
//             Some(mut parent) => {
//                 parent.buffer.push_str(buffer.as_str());
//                 self.inner.replace(*parent);
//                 None
//             }
//         }
//     }
//     pub fn get_mut(&mut self) -> &mut Scope {
//         self.inner.as_mut().unwrap()
//     }
//     pub fn get(&self) -> &Scope {
//         self.inner.as_ref().unwrap()
//     }
//     pub fn push_identifier(&mut self, identifier: Identifier) {
//         self.stack.push(identifier)
//     }
//     pub fn pop_identifier(&mut self) -> Identifier {
//         self.stack.pop().unwrap()
//     }
//     pub fn add_increment(&mut self, identifier: Identifier) {
//         self.get_mut().var_increment.push_str(format!("increment({});", identifier).as_str());
//     }
//     pub fn add_decrement(&mut self, identifier: Identifier) {
//         self.get_mut().var_decrement.push_str(format!("decrement({});", identifier).as_str());
//     }
//     pub fn push_buffer(&mut self, buffer: Buffer) {
//         self.get_mut().buffer.push_str(buffer.as_str());
//     }
//     pub fn generate_name(&mut self) -> String {
//         let name = format!("_local_{}_{}", self.get().level, self.get().num_local);
//         self.get_mut().num_local += 1;
//         name
//     }
//     pub fn generate_variable(&mut self) -> String {
//         let name = self.generate_name();
//         self.add_decrement(name.clone());
//         name
//     }
// }
// pub struct CFile {
//     fn_headers: Vec<Buffer>,
//     fn_defs: Vec<Buffer>,
// }
// impl CFile {
//     pub fn file_to_fn(file: ir::File) -> ir::FnDef {
//         FnDef {
//             identifier: "_main".to_string(),
//             args: vec![],
//             body: match file {
//                 File::Block(block) => {
//                     block
//                 }
//                 File::None => {
//                     ir::Block::WithoutExp(vec![])
//                 }
//             }
//         }
//     }
//     pub fn generate(file: ir::File) -> Buffer {
//         let fn_def = Self::file_to_fn(file);
//         let mut this = CFile {
//             fn_headers: vec![],
//             fn_defs: vec![],
//         };
//         this.fn_def(fn_def);
//         let mut buffer = Buffer::new();
//         for fn_header in this.fn_headers {
//             buffer.push_str(fn_header.as_str());
//         }
//         for fn_def in this.fn_defs {
//             buffer.push_str(fn_def.as_str());
//         }
//         buffer
//     }
//     pub fn fn_def(&mut self, fn_def: ir::FnDef) {
//         let args = args_to_string(fn_def.args);
//         let fn_header = format!("Value* {}({});", fn_def.identifier, args);
//         self.fn_headers.push(fn_header);
//         let fn_def = {
//             let mut scope = ScopeHolder::new();
//             self.block(&mut scope, fn_def.body);
//             if scope.stack.len() == 1 {
//                 scope.inner.as_mut().unwrap().return_line = Some(ReturnLine{ identifier: scope.stack.pop().unwrap() });
//             }
//             let fn_body = match scope.pop_scope() {
//                 None => panic!("we popped 1 time extra for our scope"),
//                 Some(buffer) => buffer,
//             };
//             format!("Value* {}({}){{{}}}", fn_def.identifier, args, fn_body)
//         };
//         self.fn_defs.push(fn_def);
//     }
//     pub fn block(&mut self, scope: &mut ScopeHolder, block: ir::Block) {
//         scope.push_scope();
//         match block {
//             Block::WithExp(statements, bexp) => {
//                 statements.into_iter().for_each(|statement| self.statement(scope, *statement));
//                 self.expression(scope, *bexp);
//                 let return_identifier = scope.stack.last().unwrap();
//                 scope.add_increment(return_identifier.clone());
//             }
//             Block::WithoutExp(statements) => unimplemented!(),
//         }
//         scope.pop_scope();
//     }
//     pub fn statement(&mut self, scope: &mut ScopeHolder, statement: ir::Statement) {
//         match statement {
//             Statement::FnDef(fn_def) => unimplemented!(),
//             Statement::LetStatement(_) => unimplemented!(),
//             Statement::ExpStatement(_) => unimplemented!(),
//             Statement::Block(_) => unimplemented!(),
//         }
//     }
//     pub fn expression(&mut self, scope: &mut ScopeHolder, exp: ir::Exp) {
//         match exp {
//             Exp::FnCall(_) => unimplemented!(),
//             Exp::BinaryOperation(binary_operation) => self.binary_operation(scope, binary_operation),
//             Exp::Value(value) => self.value(scope, value),
//             Exp::Variable(_) => unimplemented!(),
//         }
//     }
//     pub fn binary_operation(&mut self, scope: &mut ScopeHolder, binary_operation: ir::BinaryOperation) {
//         match binary_operation {
//             BinaryOperation { left_hand_side, operator, right_hand_side } => {
//                 self.expression(scope, *left_hand_side);
//                 self.expression(scope, *right_hand_side);
//                 let operator = match operator {
//                     BinaryOperator::Add => "ADD",
//                     BinaryOperator::Subtract => unimplemented!(),
//                     BinaryOperator::Multiply => unimplemented!(),
//                     BinaryOperator::Divide => unimplemented!(),
//                     BinaryOperator::And => unimplemented!(),
//                     BinaryOperator::Or => unimplemented!(),
//                     BinaryOperator::EqualsEquals => unimplemented!(),
//                 };
//                 let right_hand_side = scope.pop_identifier();
//                 let left_hand_side = scope.pop_identifier();
//                 let name = scope.generate_variable();
//                 scope.push_buffer(format!("Value* {}  = Number_operator({}, {}, {});", name, left_hand_side, right_hand_side, operator));
//                 scope.push_identifier(name);
//             }
//         }
//     }
//     pub fn value(&mut self, scope: &mut ScopeHolder, value: ir::Value) {
//         let generated_name = format!("_local_{}_{}", scope.get().level, scope.get().num_local);
//         scope.get_mut().num_local += 1;
//         scope.push_identifier(generated_name.clone());
//         scope.get_mut().var_declare.push_str(format!("Value* {} = {};", generated_name,
//             match value {
//                 Value::Number(number) => {
//                     format!("Number_new({})", number as i32)
//                 },
//                 Value::String(string) => {
//                     format!("String_new(\"{}\")", string)
//                 }
//                 Value::Table(_) => unimplemented!(),
//             }
//         ).as_str());
//         scope.add_decrement(generated_name);
//     }
// }
// /// for handling expressions
// #[derive(Clone)]
// pub struct ReturnLine {
//     identifier: Identifier,
// }
// fn args_to_string(args: Vec<Identifier>) -> String {
//     let mut buffer = String::default();
//     let len = args.len();
//     for (i, arg) in args.into_iter().enumerate() {
//         buffer.push_str("Value* ");
//         buffer.push_str(arg.as_str());
//         if i != len {
//             buffer.push_str(", ");
//         }
//     }
//     buffer
// }