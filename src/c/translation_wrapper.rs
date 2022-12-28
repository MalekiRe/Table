pub mod table_c_fn_generator {
    use crate::c::transpile_2::TranslationUnit;
    use crate::parser2::{Exp, FnBody, FnDef, PrimitiveValue, Statement};

    pub struct CFn {
        pub fn_header: CFnHeader,
        pub fn_body: CFnBody,
    }
    impl CFn {
        pub fn create_from(translation_unit: &mut TranslationUnit, fn_def: FnDef) -> Self {
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
                            let return_val = Self::handle_expression(&mut c_fn_body, *exp);
                            c_fn_body.statements.push(CStatement::from(format!("return {};", return_val)));
                            Self {
                                fn_header: c_fn_header,
                                fn_body: c_fn_body,
                            }
                        }
                        FnBody::Statements { statements } => {
                            for statement in statements {
                                Self::handle_statement(&mut c_fn_body, statement);
                            }
                            Self {
                                fn_header: c_fn_header,
                                fn_body: c_fn_body,
                            }
                        }
                        FnBody::Statement(statement) => {
                            Self::handle_statement(&mut c_fn_body, statement);
                            Self {
                                fn_header: c_fn_header,
                                fn_body: c_fn_body,
                            }
                        }
                        FnBody::Exp(exp) => {
                            Self::handle_expression(&mut c_fn_body, *exp);
                            Self {
                                fn_header: c_fn_header,
                                fn_body: c_fn_body,
                            }
                        }
                        FnBody::Empty => {
                            Self {
                                fn_header: c_fn_header,
                                fn_body: c_fn_body,
                            }
                        }
                    }
                }
            }
        }
        pub fn handle_statement(c_fn_body: &mut CFnBody, statement: Statement) {
            unimplemented!()
        }
        // for handling expressions, we are just going to create a local variable which would normally be inlined, and then also return that variable identifier.
        //
        pub fn handle_expression(c_fn_body: &mut CFnBody, exp: Exp) -> CIdentifier {
            match exp {
                Exp::PrimitiveValue(primitive_value) => {
                    match primitive_value {
                        PrimitiveValue::Number(_) => unimplemented!(),
                        PrimitiveValue::String(string) => {
                            let identifier = format!("_local_{}_string", c_fn_body.num_inline_var);
                            let let_statement = Self::generate_declaration_statement(identifier.clone(), PrimitiveValue::String(string));
                            c_fn_body.statements.push(let_statement);
                            identifier
                        }
                        PrimitiveValue::Boolean(_) => unimplemented!(),
                        PrimitiveValue::Function(_) => unimplemented!(),
                    }
                }
                Exp::Table(_) => unimplemented!(),
                Exp::Binary(_, _, _) => unimplemented!(),
                Exp::LocalVar(_) => unimplemented!(),
                Exp::StatementsExp(_, _) => unimplemented!(),
                Exp::FnCall(_) => unimplemented!(),
                Exp::Error => unreachable!(),
            }
        }
        pub fn generate_declaration_statement(identifier: CIdentifier, primitive_value: PrimitiveValue)  -> CStatement {
            match primitive_value {
                PrimitiveValue::Number(_) => unimplemented!(),
                PrimitiveValue::String(string) => {
                    CStatement::from(format!("TableType {} = create_string(\"{}\");", identifier, string))
                }
                PrimitiveValue::Boolean(_) => unimplemented!(),
                PrimitiveValue::Function(_) => unimplemented!(),
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
        pub fn generate_dec_header(&self) -> String {
            format!("TableType {}({});", self.fn_name, self.args_to_str())
        }
    }
    pub struct CFnBody {
        list_dec_reference_count: Vec<CIdentifier>,
        statements: Vec<CStatement>,
        num_inline_var: u32,
    }
    impl CFnBody {
        pub fn default() -> Self {
            Self {
                list_dec_reference_count: vec![],
                statements: vec![],
                num_inline_var: 0,
            }
        }
    }
    pub struct CStatement {
        buffer: String,
    }
    impl CStatement {
        fn from(string: String) -> Self {
            Self {
                buffer: string
            }
        }
    }
    impl CFn {
        pub fn get_dec_string(&self) -> String {
            let mut string = String::new();
            string.push_str(format!("TableType {} ({})", self.fn_header.fn_name, self.fn_header.args_to_str()).as_str());
            string.push_str("\n{\n");
            for statement in &self.fn_body.statements {
                string.push_str((statement.buffer.clone() + "\n").as_str())
            }
            for c_ident in &self.fn_body.list_dec_reference_count {
                string.push_str(format!("dec_ref_count({});\n", c_ident).as_str())
            }
            string.push_str("\n}\n");
            string
        }
    }
}