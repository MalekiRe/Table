use lang_c::{ast, span};
use lang_c::ast::{Declarator, DeclaratorKind, ExternalDeclaration, FunctionDefinition, Identifier, TranslationUnit};
use lang_c::span::Node;
use crate::parser2::{Exp, FnBody, FnDef, PrimitiveValue, Statement};
use crate::{ParserFile, Span};

// pub struct FileRepr {
//     pub translation_unit: TranslationUnit,
// }
// impl FileRepr {
//     pub fn new() -> Self {
//         Self {
//             translation_unit: TranslationUnit(vec![])
//         }
//     }
// }
pub struct FileRepr {
    pub fn_reps: Vec<FnRepr>,
}
pub struct FnRepr {
    pub fn_header: String,
    pub fn_dec: String,
}
impl FileRepr {
    pub fn new() -> Self {
        Self {
            fn_reps: vec![]
        }
    }
    pub fn get_file(&mut self) -> String {
        let mut ret = self.get_fn_header();
        ret.push_str(self.get_fn_dec().as_str());
        ret
    }
    pub fn get_fn_header(&mut self) -> String {
        let mut fn_headers = String::new();
        for fn_repr in &self.fn_reps {
            fn_headers.push_str(fn_repr.fn_header.as_str());
        }
        fn_headers
    }
    pub fn get_fn_dec(&mut self) -> String {
        let mut fn_decs = String::new();
        for fn_repr in &self.fn_reps {
            fn_decs.push_str(fn_repr.fn_dec.as_str());
        }
        fn_decs
    }
}
pub fn comp_file(file: ParserFile) -> FileRepr {
    let mut file_repr = FileRepr::new();
    let fn_body = match file {
        ParserFile::StatementsExp(statements, exp) => {
            FnBody::StatementsExp { statements, exp }
        }
        ParserFile::Statements(statements) => {
            FnBody::Statements { statements }
        }
    };
    let main_fn = FnDef {
        identifier: "_start".to_string(),
        args: vec![],
        fn_body,
        exported: false
    };
    define_function(&mut file_repr, main_fn);
    file_repr
}

pub fn define_function(file_repr: &mut FileRepr, fn_def: FnDef) {
    let mut fn_header = String::from("struct TableType ");
    let mut fn_dec = String::from("");
    match fn_def {
        FnDef { identifier, args, fn_body, exported } => {
            fn_header.push_str((identifier + " ").as_str());
            fn_header.push_str("(");
            for (i, arg) in args.iter().enumerate() {
                fn_header.push_str(("struct TableType ".to_string() + arg).as_str());
                if i != args.len() {
                    fn_header.push_str(",");
                }
            }
            fn_header.push_str(")");
            fn_dec.push_str(fn_header.as_str());
            fn_dec.push_str(define_function_body(fn_body).as_str());
            fn_header.push_str(";");
        }
    }
    file_repr.fn_reps.push(FnRepr{
        fn_header,
        fn_dec,
    })
}
pub fn define_function_body(fn_body: FnBody) -> String {
    let mut body = String::new();
    body.push_str("{");
    match fn_body {
        FnBody::StatementsExp { statements, exp } => {
            for statement in statements {
                body.push_str(define_statement(statement).as_str());
            }
            body.push_str("return ");
            body.push_str(define_exp(*exp).as_str());
            body.push_str(";");
        }
        FnBody::Statements { statements } => {
            unimplemented!()
        }
        FnBody::Statement(statement) => {
            unimplemented!()
        }
        FnBody::Exp(bexp) => {
            body.push_str(define_exp(*bexp).as_str());
        }
        FnBody::Empty => {}
    }
    body.push_str("}");
    body
}
pub fn define_statement(statement: Statement) -> String {
    unimplemented!()
}
pub fn define_exp(exp: Exp) -> String {
    match exp {
        Exp::PrimitiveValue(primitive_value) => {
            match primitive_value {
                PrimitiveValue::Number(number) => {
                    String::from(format!("create_big_number({}, {})", number.trunc() as u64, (number - (number.trunc())) as u64 * 100000))
                }
                PrimitiveValue::String(_) => unimplemented!(),
                PrimitiveValue::Boolean(_) => unimplemented!(),
                PrimitiveValue::Function(_) => unimplemented!(),
            }
        }
        Exp::Table(_) => unimplemented!(),
        Exp::Binary(_, _, _) => unimplemented!(),
        Exp::LocalVar(_) => unimplemented!(),
        Exp::StatementsExp(_, _) => unimplemented!(),
        Exp::FnCall(_) => unimplemented!(),
        Exp::Error => unimplemented!(),
    }
}