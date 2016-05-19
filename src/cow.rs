//! Copy-on-Write pointers.
//!
//! Thin wrappers around the standard library ref-counted pointers that clones
//! on `DerefMut` if reference count is greater than 1.

use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;

use unbox::Unbox;

pub struct RcCow<T>(pub Rc<T>);

impl<T: Clone> RcCow<T> {
    pub fn new(value: T) -> RcCow<T> {
        RcCow(Rc::new(value))
    }
}

impl<T> Clone for RcCow<T> {
    fn clone(&self) -> RcCow<T> {
        RcCow(self.0.clone())
    }
}

impl<T> Deref for RcCow<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: Clone> DerefMut for RcCow<T> {
    fn deref_mut(&mut self) -> &mut T {
        Rc::make_mut(&mut self.0)
    }
}

impl<T: Clone> Unbox for RcCow<T> {
    type Target = T;

    fn unbox(self) -> T {
        self.0.unbox()
    }
}

impl<T: fmt::Debug> fmt::Debug for RcCow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

pub struct ArcCow<T>(pub Arc<T>);

impl<T: Clone> ArcCow<T> {
    pub fn new(value: T) -> ArcCow<T> {
        ArcCow(Arc::new(value))
    }
}

impl<T> Clone for ArcCow<T> {
    fn clone(&self) -> ArcCow<T> {
        ArcCow(self.0.clone())
    }
}

impl<T> Deref for ArcCow<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: Clone> DerefMut for ArcCow<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.0)
    }
}

impl<T: Clone> Unbox for ArcCow<T> {
    type Target = T;

    fn unbox(self) -> T {
        self.0.unbox()
    }
}

impl<T: fmt::Debug> fmt::Debug for ArcCow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
