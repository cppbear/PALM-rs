// Answer 0

#[test]
fn test_compare_exchange_success() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct Inner {
        inner: AtomicUsize,
    }

    impl Inner {
        fn new(value: usize) -> Self {
            Inner {
                inner: AtomicUsize::new(value),
            }
        }

        fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
            self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
        }
    }

    let inner = Inner::new(0);
    let non_zero_val = NonZeroUsize::new(42).unwrap();
    assert_eq!(inner.compare_exchange(non_zero_val), Ok(0));
    assert_eq!(inner.inner.load(Ordering::Acquire), 42);
}

#[test]
fn test_compare_exchange_failure() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Inner {
        inner: AtomicUsize,
    }

    impl Inner {
        fn new(value: usize) -> Self {
            Inner {
                inner: AtomicUsize::new(value),
            }
        }

        fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
            self.inner.compare_exchange(1, val.get(), Ordering::Release, Ordering::Acquire)
        }
    }

    let inner = Inner::new(1);
    let non_zero_val = NonZeroUsize::new(42).unwrap();
    assert_eq!(inner.compare_exchange(non_zero_val), Err(1));
    assert_eq!(inner.inner.load(Ordering::Acquire), 1);
}

#[test]
#[should_panic]
fn test_compare_exchange_with_zero_value() {
    use std::num::NonZeroUsize;
    
    struct Inner {
        inner: std::sync::atomic::AtomicUsize,
    }

    impl Inner {
        fn new(value: usize) -> Self {
            Inner {
                inner: std::sync::atomic::AtomicUsize::new(value),
            }
        }

        fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
            self.inner.compare_exchange(0, val.get(), std::sync::atomic::Ordering::Release, std::sync::atomic::Ordering::Acquire)
        }
    }

    let inner = Inner::new(0);
    let _non_zero_val = NonZeroUsize::new(0).unwrap(); // This will cause a panic
    inner.compare_exchange(_non_zero_val);
}

