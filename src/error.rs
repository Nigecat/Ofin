use std::fmt;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (crate::error::Error::new(format!($($arg)*)));
}

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{ message: {} }} ", self.message)
    }
}
