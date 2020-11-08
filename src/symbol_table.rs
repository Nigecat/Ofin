use crate::symbols::Symbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable(HashMap<String, Box<dyn Symbol>>);

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable(HashMap::new())
    }

    /// Set a symbol in the symbol table, this will overwrite an existing symbol
    pub fn set(&mut self, key: &str, value: Box<dyn Symbol>) {
        self.0.insert(key.to_string(), value);
    }

    /// Get a symbol from the symbol table
    pub fn get(&self, key: &str) -> Option<&dyn Symbol> {
        if self.0.contains_key(key) {
            Some(&**self.0.get(key).unwrap())
        } else {
            None
        }
    }
}
