/// An ofin value
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    String(String),
    PosInteger(usize),
    NegInteger(isize),
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<isize> for Value {
    fn from(n: isize) -> Self {
        Value::NegInteger(n)
    }
}

impl From<usize> for Value {
    fn from(n: usize) -> Self {
        Value::PosInteger(n)
    }
}
