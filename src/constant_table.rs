use crate::symbols::Symbol;

#[derive(Debug)]
pub struct ConstantTable(Vec<Box<dyn Symbol>>);

impl ConstantTable {
    pub fn new() -> Self {
        ConstantTable(Vec::new())
    }

    /// Insert a value into the constant table, returns the index of the value
    pub fn insert(&mut self, value: Box<dyn Symbol>) -> usize {
        self.0.push(value);
        self.0.len() - 1
    }

    /// Retrieve a value from the constant table by index
    pub fn get(&self, index: usize) -> &dyn Symbol {
        &*self.0[index]
    }
}
