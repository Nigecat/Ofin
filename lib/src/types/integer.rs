use derive_more::{Add, Display, Div, Mul, Neg, Not, Rem, Sub};

/// TODO
#[rustfmt::skip]
#[derive(Eq, PartialEq)]                        // Equality traits
#[derive(Ord, PartialOrd)]                      // Ordering traits
#[derive(Add, Sub, Div, Mul, Not, Neg, Rem)]    // Operator traits
#[derive(Debug, Display, Copy, Clone, Hash)]    // General traits
pub struct Integer(i64);

// Implement From<T> for Number and From<Number> for T where T is an integer type
macro_rules! _impl {
    ($t: tt) => {
        impl From<$t> for Integer {
            fn from(num: $t) -> Self {
                Integer(num as i64)
            }
        }

        impl From<Integer> for $t {
            fn from(num: Integer) -> Self {
                num.0 as $t
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
