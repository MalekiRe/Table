use crate::value::Value;

pub struct Table {
    inner: Vec<TableKey>
}
pub enum TableKey {
    Str(String, Value),
    NoStr(Value),
}
impl Table {
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
    pub fn indexed_set(&mut self, index: usize, table_key: TableKey) {
        self.inner.insert(index, table_key);
    }
}