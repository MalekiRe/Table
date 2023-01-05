use std::slice::Iter;
use crate::virtual_machine::value::Value;

#[derive(Debug, PartialEq)]
pub struct Table {
    pub inner: Vec<TableKey>
}
#[derive(Debug, PartialEq)]
pub enum TableKey {
    Str(String, Value),
    NoStr(Value),
}
impl Table {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn indexed_get(&self, index: usize) -> Option<&Value> {
        match self.inner.get(index) {
            None => None,
            Some(table_key) => {
                match table_key {
                    TableKey::Str(_, value) => Some(value),
                    TableKey::NoStr(value) => Some(value),
                }
            }
        }
    }
    pub fn indexed_get_mut(&mut self, index: usize) -> Option<&mut Value> {
        match self.inner.get_mut(index) {
            None => None,
            Some(table_key) => {
                match table_key {
                    TableKey::Str(_, value) => Some(value),
                    TableKey::NoStr(value) => Some(value),
                }
            }
        }
    }
    pub fn indexed_set(&mut self, index: usize, value: Value) {
        self.inner.insert(index, TableKey::NoStr(value));
    }
}
impl Default for Table {
    fn default() -> Self {
        Self {
            inner: vec![]
        }
    }
}