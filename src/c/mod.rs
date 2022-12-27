pub mod c_compiler;

pub struct CFn {
    identifier: CIdentifier,
    args: Vec<CArgDec>,
    fn_body: CFnBody,
}
pub type CFnBody = Vec<CStatement>;
pub type CStatement = String;
pub struct CArgDec {
    identifier: CIdentifier,
    c_type: CType,
}
pub type CIdentifier = String;
enum CType {
    Float,
    Double,
    Long,
    Int,
    Short,
    Char,
    Struct(CStruct),
    Union(CUnion),
}
struct CStruct {
    identifier: CIdentifier,
    members: Vec<(CIdentifier, CType)>
}
struct CUnion {
    identifier: CIdentifier,
    members: Vec<(CIdentifier, CType)>
}