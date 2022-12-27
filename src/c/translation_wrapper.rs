use lang_c::ast;

pub struct TranslationUnit(pub Vec<ExternalDeclaration>);
impl TranslationUnit {
    fn new() -> Self {
        Self {
            0: vec![]
        }
    }
}
pub enum ExternalDeclaration {
    Declaration(Declaration),
    StaticAssert(ast::StaticAssert),
    FunctionDefinition(ast::FunctionDefinition),
}
pub struct Declaration {
    pub specifiers: Vec<DeclarationSpecifier>,
    pub declarators: Vec<ast::InitDeclarator>,
}
pub enum DeclarationSpecifier {
    StorageClass(ast::StorageClassSpecifier),
    TypeSpecifier(ast::TypeSpecifier),
    TypeQualifier(ast::TypeQualifier),
    Function(ast::FunctionSpecifier),
    Alignment(ast::AlignmentSpecifier),
    /// Vendor-specific declaration extensions that can be mixed with standard specifiers
    Extension(Vec<ast::Extension>),
}