// Answer 0

#[test]
fn test_get_when_pointer_is_null() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use core::ptr;

    struct OnceCell<T> {
        inner: AtomicPtr<T>,
    }

    impl<T> OnceCell<T> {
        pub fn get(&self) -> Option<&T> {
            let ptr = self.inner.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { &*ptr })
        }
    }

    let cell: OnceCell<i32> = OnceCell {
        inner: AtomicPtr::new(ptr::null_mut()),
    };

    let result = cell.get();
    assert_eq!(result, None);
}

