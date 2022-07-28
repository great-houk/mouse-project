use core::cell::UnsafeCell;

pub struct StaticBorrow<T: 'static> {
    inner: UnsafeCell<Option<T>>,
}

impl<T: 'static> StaticBorrow<T> {
    pub const fn new() -> Self {
        StaticBorrow {
            inner: UnsafeCell::new(None),
        }
    }

    /// Panics if used more than once
    pub fn set(&self, val: T) {
        assert!(unsafe { (*self.inner.get()).is_none() });
        unsafe { *self.inner.get() = Some(val) };
    }

    /// Only safe to use so long as there are no other references to this
    pub unsafe fn reset(&self) {
        *self.inner.get() = None;
    }

    pub unsafe fn borrow_mut(&self) -> &'static mut T {
        (*self.inner.get()).as_mut().unwrap()
    }

    /// Panics if not yet set
    pub fn borrow(&self) -> &'static T {
        assert!(unsafe { (*self.inner.get()).is_some() });
        unsafe { (*self.inner.get()).as_ref().unwrap() }
    }
}

unsafe impl<T: 'static> Send for StaticBorrow<T> {}
unsafe impl<T: 'static> Sync for StaticBorrow<T> {}
