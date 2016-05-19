//! Pointer unboxing.

use std::ptr;
use std::rc::Rc;
use std::sync::Arc;

/// Trait specifying unboxing capability of a pointer type.
pub trait Unbox {
    type Target;

    fn unbox(self) -> Self::Target;
}

impl<T> Unbox for Box<T>
    where T: Sized
{
    type Target = T;

    fn unbox(self) -> T {
        unsafe { ptr::read(Box::into_raw(self)) }
    }
}

impl<T> Unbox for Rc<T>
    where T: Clone
{
    type Target = T;

    fn unbox(mut self) -> T {
        Rc::make_mut(&mut self);
        match Rc::try_unwrap(self) {
            Ok(inner) => inner,
            _ => unreachable!(),
        }
    }
}

impl<T> Unbox for Arc<T>
    where T: Clone
{
    type Target = T;

    fn unbox(mut self) -> T {
        Arc::make_mut(&mut self);
        match Arc::try_unwrap(self) {
            Ok(inner) => inner,
            _ => unreachable!(),
        }
    }
}
