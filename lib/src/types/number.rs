use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

// FIXME: Fix negative inequality (and add unit tests for it)

/// A number.
/// 
/// TODO
#[allow(clippy::derive_hash_xor_eq)] // We do not want the rounding behaviour of 2.0 == 2 to appear in the hashed value
#[derive(Debug, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Number {
    /// The numbers before the decimal point
    integral: i64,
    /// The numbers after the decimal point
    fractional: Option<u64>,
    /// The offset of the fractional part, this should be `Some` if `self.fractional` is also `Some`
    offset: Option<u32>,
}

/// The relation of the types between two numbers
enum Relation {
    /// Number, Number
    NN,
    /// Number, Float
    NF,
    /// Float, Number
    FN,
    /// Float, Float
    FF,
}

impl Number {
    /// Detect the relation of the types between two numbers
    fn detect(a: Self, b: Self) -> Relation {
        if a.is_num() && b.is_num() {
            Relation::NN
        } else if a.is_num() && b.is_float() {
            Relation::NF
        } else if a.is_float() && b.is_num() {
            Relation::FN
        } else if a.is_float() && b.is_float() {
            Relation::FF
        } else {
            unreachable!();
        }
    }

    /// Create a number from a signed 64 bit integer
    fn from_num(val: i64) -> Self {
        Number {
            integral: val,
            fractional: None,
            offset: None,
        }
    }

    /// Create a number from a 64 bit float
    fn from_float(val: f64) -> Self {
        let integral = val.trunc() as i64;

        let fractional = val.fract();
        let length = fractional.to_string().len();
        let fractional = fractional * 10_f64.powi(length as i32);

        Number {
            integral,
            fractional: Some(fractional as u64),
            offset: Some(length as u32),
        }
    }

    /// Check if this number has no fractional part
    fn is_num(&self) -> bool {
        self.fractional == None
    }

    /// Check if this number has a fractional part (and is thus a float)
    fn is_float(&self) -> bool {
        self.fractional != None
    }

    /// Convert this number into a signed 64 bit integer
    fn into_num(self) -> i64 {
        self.integral
    }

    /// Convert this number into a 64 bit float
    fn into_float(self) -> f64 {
        match self.fractional {
            Some(fractional) => fractional as f64 / 10_u64.pow(self.offset.unwrap()) as f64,
            None => self.integral as f64,
        }
    }
}

// Implement comparison traits
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.integral == other.integral {
            match Number::detect(*self, *other) {
                Relation::NN => true,
                Relation::FF => self.fractional == other.fractional && self.offset == other.offset,
                // Allow for 2 to equal 2.0
                Relation::NF => other.fractional == Some(0),
                Relation::FN => self.fractional == Some(0),
            }
        } else {
            false
        }
    }
}

// Implement the operator trait for $trait using operator $op
macro_rules! _impl_op {
    ($trait: ident, $f: ident, $op: tt) => {
        impl $trait for Number {
            type Output = Self;

            fn $f(self, other: Self) -> Self {
                // Perform the conversion on the integral
                let integral = self.integral $op other.integral;

                let fractional = match Number::detect(self, other) {
                    Relation::NN => 0_f64,
                    Relation::FN => self.fractional.unwrap() as f64 / 10_u64.pow(self.offset.unwrap()) as f64,
                    Relation::NF => other.fractional.unwrap() as f64 / 10_u64.pow(other.offset.unwrap()) as f64,
                    Relation::FF => {
                        let fractional = self.fractional.unwrap() $op other.fractional.unwrap();
                        fractional as f64 / 10_u64.pow(self.offset.unwrap()) as f64
                    }
                };

                if fractional == 0_f64 {
                    Number::from(integral)
                } else {
                    Number::from(integral as f64 + fractional)
                }
            }
        }
    };
}

// Implement operator traits
_impl_op!(Add, add, +);
_impl_op!(Sub, sub, -);
_impl_op!(Mul, mul, *);
_impl_op!(Div, div, /);

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number {
            integral: -self.integral,
            fractional: self.fractional,
            offset: self.offset,
        }
    }
}

// Implement From<T> for Number and From<Number> for T where T is an integer
macro_rules! _impl {
    ($t: tt) => {
        impl From<$t> for Number {
            fn from(val: $t) -> Self {
                Number::from_num(val as i64)
            }
        }

        impl From<Number> for $t {
            fn from(num: Number) -> Self {
                num.into_num() as $t
            }
        }
    };
}

// Implement conversion traits
_impl!(u8);
_impl!(i8);
_impl!(u16);
_impl!(i16);
_impl!(u32);
_impl!(i32);
_impl!(u64);
_impl!(i64);
_impl!(usize);
_impl!(isize);

// Manually implement float conversion traits
impl From<f32> for Number {
    fn from(val: f32) -> Self {
        Number::from_float(val as f64)
    }
}

impl From<Number> for f32 {
    fn from(num: Number) -> Self {
        num.into_float() as f32
    }
}

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Number::from_float(val)
    }
}

impl From<Number> for f64 {
    fn from(num: Number) -> Self {
        num.into_float()
    }
}
