use super::Integer;
use derive_more::Display;

#[derive(Display, Debug, Hash, Eq, PartialEq, Clone)]
enum ValueType {
    Integer(Integer),
}

/// A generic value.
///
/// This could be any of the standard library types.
#[derive(Display, Debug, Hash, Eq, PartialEq, Clone)]
pub struct Value(ValueType);

impl Value {
    /// Convert this value into an integer
    pub fn into_integer(self) -> Option<Integer> {
        match self.0 {
            ValueType::Integer(i) => Some(i),
            // _ => None,
        }
    }
}

impl From<Integer> for Value {
    fn from(num: Integer) -> Self {
        Value(ValueType::Integer(num))
    }
}
