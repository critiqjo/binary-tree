//! Pointer unboxing.

use std::ptr;
use std::rc::Rc;
use std::sync::Arc;

/// Trait specifying unboxing capability of a pointer type.
pub trait Unbox<T: Sized> {
    fn unbox(self) -> T;
}

impl<T> Unbox<T> for Box<T>
    where T: Sized
{
    fn unbox(self) -> T {
        unsafe { ptr::read(Box::into_raw(self)) }
    }
}

impl<T> Unbox<T> for Rc<T>
    where T: Clone
{
    fn unbox(mut self) -> T {
        Rc::make_mut(&mut self);
        match Rc::try_unwrap(self) {
            Ok(inner) => inner,
            _ => unreachable!(),
        }
    }
}

impl<T> Unbox<T> for Arc<T>
    where T: Clone
{
    fn unbox(mut self) -> T {
        Arc::make_mut(&mut self);
        match Arc::try_unwrap(self) {
            Ok(inner) => inner,
            _ => unreachable!(),
        }
    }
}
