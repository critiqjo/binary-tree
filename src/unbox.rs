use std::ptr;
use std::rc::Rc;
use std::sync::Arc;

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
        if let Ok(inner) = Rc::try_unwrap(self) {
            inner
        } else {
            unreachable!("Unboxing failed after make_mut!")
        }
    }
}

impl<T> Unbox<T> for Arc<T>
    where T: Clone
{
    fn unbox(mut self) -> T {
        Arc::make_mut(&mut self);
        if let Ok(inner) = Arc::try_unwrap(self) {
            inner
        } else {
            unreachable!("Unboxing failed after make_mut!")
        }
    }
}
