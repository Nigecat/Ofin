//! Random number utilities.

/// Generate a random positive number.
pub fn randomPosnum() -> usize {
    rand::random()
}

/// Generate a random positive or negative number.
pub fn randomNum() -> isize {
    rand::random()
}

/// Generate a random boolean
pub fn randomBool() -> bool {
    rand::random()
}
