use std::collections::HashMap;
use crate::compiler::ir::IdentifierT;
use crate::compiler::parser2::vm3::value::StackValue;


#[derive(Debug, Clone)]
pub struct Table {
    identifier_map: HashMap<IdentifierT, u32>,
    values: Vec<StackValue>,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            identifier_map: Default::default(),
            values: vec![]
        }
    }
}
impl Table {
    pub fn get_with_ident(&self, identifier: &str) -> StackValue {
        self.values[*self.identifier_map.get(identifier).unwrap() as usize]
    }
    pub fn get(&self, index: u32) -> StackValue {
        self.values[index as usize]
    }
    pub fn push_with_ident(&mut self, identifier: IdentifierT, value: StackValue) -> u32{
        let index = self.values.len() as u32;
        self.identifier_map.insert(identifier, index);
        self.values.push(value);
        index
    }
    pub fn push(&mut self, value: StackValue) -> u32 {
        let index = self.values.len() as u32;
        self.values.push(value);
        index
    }
    pub fn replace_with_ident(&mut self, identifier: &str, value: StackValue) -> u32 {
        let index = *self.identifier_map.get(identifier).unwrap();
        self.values[index as usize] = value;
        index
    }
    pub fn replace(&mut self, index: u32, value: StackValue) {
        self.values[index as usize] = value;
    }
    pub fn remove_with_ident(&mut self, identifier: &str) {
        let index = self.identifier_map.remove(identifier).unwrap();
        self.values[index as usize] = StackValue::Nil;
    }
    pub fn remove(&mut self, index: u32) {
        self.values[index as usize] = StackValue::Nil;
    }
}