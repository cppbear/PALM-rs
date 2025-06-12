// Answer 0

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct OnceCell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> OnceCell<T> {
        pub fn new() -> Self {
            Self {
                inner: AtomicPtr::new(std::ptr::null_mut()),
            }
        }

        pub fn get(&self) -> Option<&T> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    #[test]
    fn test_get_when_ptr_is_null() {
        let cell: OnceCell<i32> = OnceCell::new();
        assert_eq!(cell.get(), None);
    }
}

