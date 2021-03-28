#[derive(Debug, Copy, Clone, Hash)]
pub struct Number {
    /// The numbers before the decimal point
    integral: i64,
    /// The numbers after the decimal point
    fractional: Option<u64>,
    /// The offset of the fractional part, this should be `Some` if `self.fractional` is also `Some`
    offset: Option<u32>,
}

impl Number {
    /// Check if this number has no fractional part
    fn is_num(&self) -> bool {
        self.fractional == None
    }

    /// Check if this number has a fractional part (and is thus a float)
    fn is_float(&self) -> bool {
        self.fractional != None
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

    /// Convert this number into a signed 64 bit integer
    fn into_num(&self) -> i64 {
        self.integral
    }

    /// Convert this number into a 64 bit float
    fn into_float(&self) -> f64 {
        match self.fractional {
            Some(fractional) => fractional as f64 / 10_u64.pow(self.offset.unwrap()) as f64,
            None => self.integral as f64,
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
