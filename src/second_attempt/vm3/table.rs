pub struct Table {
    inner: Vec<(Option<String>, Value)>,
}
impl Table {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_with_index(&self, index: usize) -> Option<&Value> {
        self.inner.get(index).unwrap()
    }
}
impl Default for Table {
    fn default() -> Self {
        Self {
            inner: vec![]
        }
    }
}