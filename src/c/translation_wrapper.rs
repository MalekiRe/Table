mod table_c_fn_generator {
    use std::os::linux::raw::stat;
    use crate::parser2::{Exp, FnBody, FnDef, PrimitiveValue, Statement};

    pub struct CFn {
        fn_header: CFnHeader,
        fn_body: CFnBody,
    }
    impl CFn {
        pub fn create_from(fn_def: FnDef) -> Self {
            match fn_def {
                FnDef { identifier, args, fn_body, exported } => {
                    let c_fn_header = CFnHeader {
                        fn_name: identifier,
                        args
                    };
                    let mut c_fn_body = CFnBody::default();
                    match fn_body {
                        FnBody::StatementsExp { statements, exp } => {
                            for statement in statements {
                                Self::handle_statement(&mut c_fn_body, statement);
                            }
                            Self::handle_expression(&mut c_fn_body, *exp);
                        }
                        FnBody::Statements { statements } => {
                            for statement in statements {
                                Self::handle_statement(&mut c_fn_body, statement);
                            }
                        }
                        FnBody::Statement(statement) => {
                            Self::handle_statement(&mut c_fn_body, statement);
                        }
                        FnBody::Exp(exp) => {
                            Self::handle_exp(&mut c_fn_body, *exp);
                        }
                        FnBody::Empty => {}
                    }
                }
            }
            unimplemented!()
        }
        fn handle_statement(c_fn_body: &mut CFnBody, statement: Statement) {
            unimplemented!()
        }
        fn handle_expression(c_fn_body: &mut CFnBody, exp: Exp) {
            match exp {
                Exp::PrimitiveValue(primitive_value) => {
                    match primitive_value {
                        PrimitiveValue::Number(_) => {}
                        PrimitiveValue::String(string) => {
                            
                        }
                        PrimitiveValue::Boolean(_) => {}
                        PrimitiveValue::Function(_) => {}
                    }
                }
                Exp::Table(_) => {}
                Exp::Binary(_, _, _) => {}
                Exp::LocalVar(_) => {}
                Exp::StatementsExp(_, _) => {}
                Exp::FnCall(_) => {}
                Exp::Error => {}
            }
        }
    }
    pub type CIdentifier = String;
    pub struct CFnHeader {
        fn_name: CIdentifier,
        args: Vec<CIdentifier>
    }
    impl CFnHeader {
        pub fn args_to_str(&self) -> String {
            let mut string = String::new();
            for (i, arg) in self.args.iter().enumerate() {
                string.push_str(format!("struct TableType {}", arg).as_str());
                if i != self.args.len() {
                    string.push_str(",");
                }
            }
            string
        }
    }
    pub struct CFnBody {
        list_dec_reference_count: Vec<CIdentifier>,
        statements: Vec<CStatement>,
    }
    impl CFnBody {
        pub fn default() -> Self {
            Self {
                list_dec_reference_count: vec![],
                statements: vec![]
            }
        }
    }
    pub struct CStatement {
        buffer: String,
    }
    impl CFn {
        pub fn get_dec_string(&self) -> String {
            let mut string = String::new();
            string.push_str(format!("{} ({})", self.fn_header.fn_name, self.fn_header.args_to_str()).as_str());
            string.push_str("\n{\n");
            for statement in self.fn_body.statements {
                string.push_str((statement.buffer + "\n").as_str())
            }
            for c_ident in self.fn_body.list_dec_reference_count {
                string.push_str(format!("dec_ref_count({});\n", c_ident).as_str())
            }
            string
        }
    }
}