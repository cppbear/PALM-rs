// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestStruct {
        inner: AtomicUsize,
    }

    impl TestStruct {
        fn new(value: usize) -> Self {
            TestStruct {
                inner: AtomicUsize::new(value),
            }
        }

        fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
            self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
        }
    }

    #[test]
    fn test_compare_exchange_success() {
        let test_struct = TestStruct::new(0);
        let non_zero_value = NonZeroUsize::new(42).unwrap();
        let result = test_struct.compare_exchange(non_zero_value);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_compare_exchange_failure() {
        let test_struct = TestStruct::new(10);
        let non_zero_value = NonZeroUsize::new(5).unwrap();
        let result = test_struct.compare_exchange(non_zero_value);
        assert_eq!(result, Err(10));
    }

    #[test]
    #[should_panic]
    fn test_compare_exchange_panic_on_zero() {
        let test_struct = TestStruct::new(0);
        let _ = test_struct.compare_exchange(NonZeroUsize::new(0).unwrap());
    }
}

