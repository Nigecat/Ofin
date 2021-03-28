use super::Integer;
use derive_more::Display;
use std::convert::TryFrom;

/// A generic value.
///
/// This could be any of the standard library types.
#[derive(Display, Debug, Hash, Eq, PartialEq, Clone)]
pub enum Value {
    /// An integer
    Integer(Integer),
}

// impl Value {
//     /// Check if this value is an integer
//     pub fn is_integer(&self) -> bool {
//         match self {
//             Value::Integer(_) => true,
//             // _ => false,
//         }
//     }
// }

impl From<Integer> for Value {
    fn from(num: Integer) -> Self {
        Value::Integer(num)
    }
}

impl TryFrom<Value> for Integer {
    type Error = ();

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        match val {
            Value::Integer(i) => Ok(i),
            // _ => Err(()),
        }
    }
}
