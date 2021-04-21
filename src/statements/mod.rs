mod using;

use using::Using;

pub enum Statement {
    Using(Using),
}

pub mod imp {
    pub trait Statement: std::fmt::Debug {
        /// Check if the string is a valid instance of this statement
        fn check(s: &str) -> bool;

        /// Convert a string into this statement
        /// If `Statement::check(s)` returns `false` then `Statement::convert(s)` is allowed to panic
        fn convert(s: String) -> Self;
    }
}
