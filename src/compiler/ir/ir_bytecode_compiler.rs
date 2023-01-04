use crate::Bytecode;
use crate::compiler::ir;
use crate::compiler::ir::{BStatement, ExpBlock, File, Statement, StatementBlock};
use crate::misc::VecTuple1;

pub struct IRCompiler {
    bytecode: Vec<Bytecode>,
}
impl IRCompiler {
    pub fn compiler(file: File) -> Vec<Bytecode> {
        let mut this = Self {
            bytecode: vec![]
        };
        this.compile_file(file);
        this.bytecode
    }
    fn compile_file(&mut self, file: File) {
        // match file {
        //     File::ExpBlock(exp_block) => self.exp_block(exp_block),
        //     File::StatementBlock(statement_block) => self.statement_block(statement_block),
        //     File::Empty => {}
        // }
    }
    // fn exp_block(&mut self, exp_block: ExpBlock) {
    //     unimplemented!()
    // }
    // fn statement_block(&mut self, statement_block: StatementBlock) {
    //     match statement_block {
    //         StatementBlock(statements) => {
    //             match statements {
    //                 VecTuple1(first, rest) => {
    //                     self.statement(*first);
    //                     for statement in rest {
    //                         self.statement(*statement);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // fn statement(&mut self, statement: Statement) {
    //     unimplemented!()
    // }
}