use super::Expression;

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function { name }
    }
}
