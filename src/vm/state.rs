use super::{Function, Value};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct State {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
}
