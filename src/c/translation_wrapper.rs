mod table_c_fn_generator {
    pub struct CFn {
        fn_header: CFnHeader,
        fn_body: CFnBody,
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