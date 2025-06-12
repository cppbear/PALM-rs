// Answer 0

#[test]
fn test_get_non_zero() {
    use std::num::NonZeroUsize;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestStruct {
        inner: AtomicUsize,
    }

    impl TestStruct {
        pub fn new(value: usize) -> Self {
            Self {
                inner: AtomicUsize::new(value),
            }
        }

        pub fn get(&self) -> Option<NonZeroUsize> {
            let val = self.inner.load(Ordering::Acquire);
            NonZeroUsize::new(val)
        }
    }

    let test_instance = TestStruct::new(1);
    assert!(test_instance.get().is_some());
    assert_eq!(test_instance.get(), NonZeroUsize::new(1));
}

#[test]
fn test_get_zero() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestStruct {
        inner: AtomicUsize,
    }

    impl TestStruct {
        pub fn new(value: usize) -> Self {
            Self {
                inner: AtomicUsize::new(value),
            }
        }

        pub fn get(&self) -> Option<NonZeroUsize> {
            let val = self.inner.load(Ordering::Acquire);
            NonZeroUsize::new(val)
        }
    }

    let test_instance = TestStruct::new(0);
    assert!(test_instance.get().is_none());
}

