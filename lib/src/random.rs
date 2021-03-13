//! Random number utilities.

/// Generate a random positive number.
pub fn random_posnum() -> usize {
    rand::random()
}

/// Generate a random positive or negative number.
pub fn random_num() -> isize {
    rand::random()
}

/// Generate a random boolean
pub fn random_bool() -> bool {
    rand::random()
}