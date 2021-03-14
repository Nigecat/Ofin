use std::any::Any;

pub type Transmutable = Box<dyn Transmute<Box<dyn Any>>>;

pub trait Transmute<T>: Any {
    /// Convert `self` to `T`
    fn transmute(&self) -> Option<&T>;
}

impl<A: Any, T: 'static> Transmute<T> for A {
    fn transmute(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}
