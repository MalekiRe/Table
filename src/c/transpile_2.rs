use crate::c::translation_wrapper::table_c_fn_generator::CFn;
use crate::parser2::{FnBody, FnDef};
use crate::ParserFile;

pub fn transpile(parser_file: ParserFile) -> String {
    match parser_file {
        ParserFile::StatementsExp(statements, exp) => {
            let end_file = "\nvoid _start() {\n\tprint_table_type(_main());\n}";

            let main_fn = FnDef {
                identifier: "_main".to_string(),
                args: vec![],
                fn_body: FnBody::StatementsExp { statements, exp },
                exported: false
            };
            let mut file = TranslationUnit::transpile(main_fn);
            file.push_str(end_file);
            file
        }
        ParserFile::Statements(statements) => unimplemented!(),
    }
}

pub struct TranslationUnit {
    pub c_funcs: Vec<CFn>,
}
impl TranslationUnit {
    pub fn transpile(entry_point: FnDef) -> String {
        let mut this = Self {
            c_funcs: vec![]
        };
        let temp = CFn::create_from(&mut this, entry_point);
        this.c_funcs.push(temp);
        this.generate_file()
    }
    pub fn generate_file(&self) -> String {
        let mut buffer = String::new();
        for function in &self.c_funcs {
            buffer.push_str((function.fn_header.generate_dec_header() + "\n").as_str())
        }
        for function in &self.c_funcs {
            buffer.push_str((function.get_dec_string() + "\n").as_str());
        }
        buffer
    }
}