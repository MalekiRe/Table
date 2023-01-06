use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::default::Default;
use crate::compiler::parser::{literal_value, table_operation};
use crate::{Exp, LetStatement, StatementBlock};
use crate::ir::{BinaryOp, BinaryOperation, File, IdentifierT, LiteralValue, MathOp, OptionalStatementBlock, ReassignmentStatement, Statement, TableKeyTemp, TableLiteral, TableOperation};
use crate::ir::ir_bytecode_compiler::{FnHeader, Variable};
use crate::register_machine::stack_value::StackValue;
use crate::register_machine::vm::{Bytecode, Chunk};
use crate::register_machine::vm::Bytecode::{AddPop, AllocString, AllocTable, GetTableNum, GetTableStr, LoadConstant, PeekLocal, PushChar, PushTableNum, PushTableStr};

pub struct Scope {
    variables: HashMap<IdentifierT, Location>,
}
#[derive(Debug, Clone, Copy)]
pub enum Location {
    Local(usize),
    Heap(usize),
    Constant(usize),
}
pub struct ScopeHolder(Vec<Scope>);
impl ScopeHolder {
    pub fn add_variable(&mut self, variable_name: IdentifierT, location: Location) {
        self.0.last_mut().unwrap().variables.insert(variable_name, location);
    }
    pub fn find_variable(&mut self, variable_name: IdentifierT) -> Option<Location> {
        let mut index = self.0.len();
        loop {
            if index == 0 {
                break;
            }
            index -= 1;
            match self.0[index].variables.get(variable_name.as_str()) {
                None => (),
                Some(thing) => {
                    return Some(*thing)
                }
            }
        }
        None
    }
    pub fn push(&mut self) {
        self.0.push(Scope::default())
    }
    pub fn pop(&mut self) {
        self.0.pop();
    }
}
impl Default for ScopeHolder {
    fn default() -> Self {
        Self {
            0: vec![Scope::default()]
        }
    }
}
impl Default for Scope {
    fn default() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}
pub struct IRCompiler {
    bytecode: Vec<Bytecode>,
    consts: Vec<StackValue>,
    const_strs: Vec<String>,
    scope_holder: ScopeHolder,
    stack_size: usize,
}
impl IRCompiler {
    pub fn compile(file: File) -> Chunk {
        let mut this = Self {
            bytecode: vec![],
            consts: Default::default(),
            const_strs: vec![],
            scope_holder: Default::default(),
            stack_size: 0,
        };
        this.file(file);
        match this {
            IRCompiler { bytecode, consts, .. } => {
                Chunk::from(bytecode, consts)
            }
        }
    }
    pub fn file(&mut self, file: File) {
        match file {
            File::JustStatements(_) => {}
            File::StatementExp(statements, exp) => {
                for statement in statements {
                    self.statement(*statement);
                }
                self.exp(*exp);
            }
            File::Empty => {}
        }
    }
    pub fn statement(&mut self, statement: Statement) {
        match statement {
            Statement::FnDec(_) => todo!(),
            Statement::FnImport(_) => todo!(),
            Statement::LetStatement(let_statement) => self.let_statement(let_statement),
            Statement::ExpStatement(_) => todo!(),
            Statement::StatementBlock(optional_statement_block) => self.optional_statement_block(optional_statement_block),
            Statement::UnaryPostfixOperation(_) => todo!(),
            Statement::ReassignmentStatement(reassignment_statement) => self.reassignment_statement(reassignment_statement),
        }
    }
    pub fn reassignment_statement(&mut self, reassignment_statement: ReassignmentStatement) {
        match reassignment_statement {
            ReassignmentStatement { identifier, lhs } => {
                let location = self.scope_holder.find_variable(identifier.clone()).expect(format!("variable '{}' doesn't exist or isn't in scope", identifier).as_str());
                self.exp(*lhs);
                match location {
                    Location::Local(local) => {
                        self.bytecode.push(Bytecode::SetLocal(local as u32))
                    }
                    Location::Heap(_) => todo!(),
                    Location::Constant(_) => todo!(),
                }
            }
        }
    }
    pub fn optional_statement_block(&mut self, statement_block: OptionalStatementBlock) {
        match statement_block {
            OptionalStatementBlock::StatementBlock(statement_block) => self.statement_block(statement_block),
            OptionalStatementBlock::Empty => {}
        }
    }
    pub fn statement_block(&mut self, statement_block: StatementBlock) {
        self.scope_holder.push();
        match statement_block {
            StatementBlock(statements) => {
                self.statement(*statements.0);
                for statement in statements.1 {
                    self.statement(*statement)
                }
            }
        }
        self.scope_holder.pop();
    }
    pub fn let_statement(&mut self, let_statement: LetStatement) {
        match let_statement {
            LetStatement { identifier, lhs } => {
                self.exp(*lhs);
                self.bytecode.push(Bytecode::PushLocal);
                self.bytecode.push(Bytecode::Pop);

                self.scope_holder.add_variable(identifier, Location::Local(self.stack_size));
                self.stack_size += 1;
            }
        }
    }
    pub fn exp(&mut self, exp: Exp) {
        match exp {
            Exp::ExpBlock(_) => todo!(),
            Exp::LiteralValue(value) => self.literal_value(value),
            Exp::FnCall(_) => todo!(),
            Exp::TableOperation(table_operation) => self.table_operation(table_operation),
            Exp::Variable(variable) => self.variable(variable),
            Exp::UnaryPrefixOperation(_) => todo!(),
            Exp::BinaryOperation(binary_operation) => self.binary_operation(binary_operation),
        }
    }
    pub fn table_operation(&mut self, table_operation: TableOperation) {
        match table_operation {
            TableOperation::TableIndexing { table, index } => {
                self.exp(*table);
                // now we have the heap index for the table pushed onto the top of the stack.
                self.exp(*index);
                // now we have the index at the top of the stack as well.
                self.bytecode.push(GetTableNum)
            }
            TableOperation::TableMethodCalling { .. } => todo!(),
            TableOperation::TableFieldAccess { table, field } => {
                // table is now on top of the stack
                self.exp(*table);
                // then the stack is `string` `table`
                self.string_literal(field);
                self.bytecode.push(GetTableStr);

            },
            TableOperation::TableStaticFuncCalling { .. } => todo!(),
        }
    }
    pub fn variable(&mut self, variable: IdentifierT) {
        match self.scope_holder.find_variable(variable).unwrap() {
            Location::Local(local) => {
                self.bytecode.push(PeekLocal(local as u32))
            }
            Location::Heap(heap) => todo!(),
            Location::Constant(constant) => todo!(),
        }
    }
    pub fn binary_operation(&mut self, binary_operation: BinaryOperation) {
        match binary_operation {
            BinaryOperation { lhs, op, rhs } => {
                self.exp(*lhs);
                self.exp(*rhs);
                match op {
                    BinaryOp::Math(math) => match math {
                        MathOp::Add => {
                            self.bytecode.push(AddPop);
                        }
                        MathOp::Subtract => todo!(),
                        MathOp::Multiply => todo!(),
                        MathOp::Divide => todo!(),
                        MathOp::Modulo => todo!(),
                        MathOp::AddEqual => todo!(),
                        MathOp::MinusEqual => todo!(),
                        MathOp::MultiplyEqual => todo!(),
                        MathOp::DivideEqual => todo!(),
                        MathOp::ModuloEqual => todo!(),
                    }
                    BinaryOp::Equality(_) => todo!(),
                }
            }
        }
    }
    pub fn literal_value(&mut self, literal_value: LiteralValue) {
        let stack_value = match literal_value {
            LiteralValue::Decimal(decimal) => Some(StackValue::Number(decimal as f32)),
            LiteralValue::Integer(integer) => Some(StackValue::Number(integer as f32)),
            LiteralValue::String(string) => {self.string_literal(string); None},
            LiteralValue::Table(table_literal) => { self.table_literal(table_literal); None}
            LiteralValue::Boolean(boolean) => Some(StackValue::Boolean(boolean)),
        };
        match stack_value {
            None => {}
            Some(stack_value) => {
                self.add_constant(stack_value);
            }
        }
    }
    fn add_constant(&mut self, stack_value: StackValue) {
        if !self.consts.contains(&stack_value) {
            self.consts.push(stack_value);
        }
        self.bytecode.push(LoadConstant((self.consts.iter().position(|&v| v == stack_value)).unwrap() as u32))
    }
    fn table_literal(&mut self, table_literal: TableLiteral) {
        self.bytecode.push(AllocTable);
        for (i, item) in table_literal.into_iter().enumerate() {
            match item {
                TableKeyTemp { ident, exp } => {
                    match ident {
                        None => {
                            self.exp(*exp);
                            self.bytecode.push(PushTableNum);
                        }
                        Some(ident) => {
                            // puts the string pointer is on the top of the stack.
                            self.string_literal(ident);
                            self.exp(*exp);
                            self.bytecode.push(PushTableStr);
                        }
                    }
                    self.bytecode.push(Bytecode::Pop);
                }
            }
        }
    }
    fn string_literal(&mut self, string: String) {
        self.bytecode.push(AllocString);
        for char in string.chars() {
            self.add_constant(StackValue::Number(char as u32 as f32));
            self.bytecode.push(PushChar);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ir::ir_bytecode_compiler::parse_file;
    use crate::register_machine::ir_bytecode_compiler::IRCompiler;
    use crate::register_machine::stack_value::StackValue;
    use crate::register_machine::vm::{HeapValue, Vm};

    #[test]
    fn first() {
        let mut vm = Vm::new();
        let file = parse_file("1").unwrap();
        let chunk = IRCompiler::compile(file);
        vm.load(chunk);
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(1.0)])
    }
    #[test]
    fn second() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("1+2").unwrap()));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(3.0)])
    }
    #[test]
    fn let_statement() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("let x = 1; let y = 2; let z = x + 1; y + z").unwrap()));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(4.0)])
    }
    #[test]
    fn table_dec_no_str() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("[1, false, 0.2]").unwrap()));
        vm.run();
    }
    #[test]
    fn table_indexing() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("[1, false, 0.2]@1").unwrap()));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Boolean(false)])
    }
    #[test]
    fn complex_table_access() {
        let mut vm = Vm::new();
        let str = r#"
        let my_table = [1, false, 0.2];
        let some_index = 0;
        let z = 2;
        z + my_table@some_index
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        vm.run();
        assert_eq!(vm.chunk().stack, vec![StackValue::Number(3.0)])
    }
    #[test]
    fn simple_str_test() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file("\"hi\"").unwrap()));
        vm.run();
        let str: &String = vm.get_heap(0).try_into().unwrap();
        assert_eq!(str, &String::from("hi"))
    }
    #[test]
    fn table_str_test() {
        let mut vm = Vm::new();
        vm.load(IRCompiler::compile(parse_file(r#"
            let my_str = "hello world!";
            let my_table = [my_str, 1, "yo yo yo"];
            my_table@0
        "#).unwrap()));
        vm.run();
        let str_pos: usize = vm.pop().try_to_str_index().unwrap();
        let str: &String = vm.get_heap(str_pos as u32).try_into().unwrap();
        assert_eq!(str, &String::from("hello world!"));
    }
    #[test]
    fn table_str_1() {
        let mut vm = Vm::new();
        let str = r#"
            let my_table = [some_thing: 1, false, anotherthing: "yo"];
            my_table@0
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        vm.run();
        assert_eq!(vm.pop(), StackValue::Number(1.0));
    }
    #[test]
    fn table_field_access() {
        let mut vm = Vm::new();
        let str = r#"
            let my_table = [some_thing: 1, false, anotherthing: "yo"];
            1 + my_table.some_thing
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        vm.run();
        assert_eq!(vm.pop(), StackValue::Number(2.0));
    }
    #[test]
    fn reassignment() {
        let mut vm = Vm::new();
        let str = r#"
            let foo = 1;
            foo = 2;
            foo
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        vm.run();
        assert_eq!(vm.pop(), StackValue::Number(2.0));
    }
    #[test]
    fn scoping_2() {
        let mut vm = Vm::new();
        let str = r#"
        let foo =  1;
        {
            let bar = foo;
        }
        foo
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        vm.run();
    }
    #[test]
    fn scoping_3() {
        let mut vm = Vm::new();
        let str = r#"
            {
                let x = 1;
            }
        "#;
        let file = parse_file(str).unwrap();
        vm.load(IRCompiler::compile(file));
        vm.run();
    }
    #[test]
    fn scoping() {
        let mut vm = Vm::new();
        let str = r#"
            let foo = 1;
            let bar = 0;
            {
                let foo = 2;
                {
                    bar = foo;
                }
            }
            bar
        "#;
        vm.load(IRCompiler::compile(parse_file(str).unwrap()));
        //vm.run();
        //assert_eq!(vm.pop(), StackValue::Number(2.0));
    }
}