use crate::second_attempt::ir;
use crate::second_attempt::ir::Value;

pub type TIdentifier = String;
pub type CIdentifier = String;
pub type Buffer = String;

pub fn generate_inline_identifier(level: u32, num_local: &mut u32) -> CIdentifier {
    let name = format!("_inline_{}_{}", level, num_local);
    use std::ops::AddAssign;
    num_local.add_assign(&1);
    name
}
pub fn generate_function_identifier(fn_name: TIdentifier, level: u32) -> CIdentifier {
    format!("_{}_{}", fn_name, level)
}
pub fn generate_variable_identifier(variable_name: TIdentifier, level: u32) -> CIdentifier {
    format!("_{}_{}", variable_name, level)
}
pub fn generate_binary_operation(identifier: CIdentifier, lhs: Buffer, rhs: Buffer, operator: String) -> Buffer {
    format!("Value* {}  = Number_operator({}, {}, {});", identifier, lhs, rhs, operator)
}
pub fn generate_function_def(identifier: CIdentifier, args: Buffer, fn_body: Buffer) -> Buffer {
    format!("Value* {}({}){{{}}}", identifier, args, fn_body)
}
pub fn generate_function_header(identifier: CIdentifier, args: Buffer) -> Buffer {
    format!("Value* {}({});", identifier, args)
}
pub fn generate_increment(identifier: CIdentifier) -> Buffer {
    format!("increment({});", identifier)
}
pub fn generate_decrement(identifier: CIdentifier) -> Buffer {
    format!("decrement({});", identifier)
}
pub fn generate_return_line(identifier: CIdentifier) -> Buffer {
    format!("return({});", identifier)
}
pub fn generate_value_new(value: ir::Value) -> Buffer {
    match value {
        Value::Number(number) => format!("Number_new({})", number as i32),
        Value::String(string) => format!("String_new(\"{}\")", string),
        Value::Table(_) => unimplemented!()
    }
}
pub fn generate_variable_declaration(identifier: CIdentifier, rhs: Buffer) -> Buffer {
    format!("Value* {} = {};", identifier, rhs)
}
pub fn generate_closure_declaration(inline_identifier: CIdentifier, fn_identifier: CIdentifier, closure_idents: Vec<CIdentifier>) -> Buffer {
    let mut buffer = Buffer::default();
    buffer.push_str(format!("Closure {};", inline_identifier).as_str());
    buffer.push_str(format!("{}.args = malloc(sizeof(Value*) * {});", inline_identifier, closure_idents.len()).as_str());
    buffer.push_str(format!("{}.p = &{};", inline_identifier, fn_identifier).as_str());
    for (i, arg) in closure_idents.into_iter().enumerate() {
        buffer.push_str(format!("{}.args[{}] = {};", inline_identifier, i, arg).as_str());
    }
    buffer.push_str(format!("Value* {} = Closure_new({});", fn_identifier, inline_identifier).as_str());
    buffer
}

pub fn args_to_string(args: Vec<TIdentifier>) -> String {
    let mut buffer = String::default();
    buffer.push_str("Value** args");
    let len = args.len();
    if len != 0 {
        buffer.push_str(",");
    }
    for (i, arg) in args.into_iter().enumerate() {
        buffer.push_str("Value* ");
        buffer.push_str(arg.as_str());
        if i != len-1 {
            buffer.push_str(", ");
        }
    }
    buffer
}
pub fn call_args_to_string(closure_name: String, args: Vec<CIdentifier>) -> String {
    let mut buffer = String::default();
    buffer.push_str(format!("{}->variant.closure->args", closure_name).as_str());
    let len = args.len();
    if len != 0 {
        buffer.push_str(",");
    }
    for (i, arg) in args.into_iter().enumerate() {
        buffer.push_str(arg.as_str());
        if i != len-1 {
            buffer.push_str(", ");
        }
    }
    buffer
}