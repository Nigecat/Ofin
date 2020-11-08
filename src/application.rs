use crate::constant_table::ConstantTable;
use crate::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct Application {
    pub symbols: SymbolTable,
    pub constants: ConstantTable,
}

impl Application {
    pub fn new() -> Self {
        Application {
            symbols: SymbolTable::new(),
            constants: ConstantTable::new(),
        }
    }
}
