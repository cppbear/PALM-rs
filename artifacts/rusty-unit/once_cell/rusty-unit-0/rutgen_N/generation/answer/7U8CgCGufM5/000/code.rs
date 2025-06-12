// Answer 0

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct MyStruct {
        inner: AtomicUsize,
    }
    
    impl MyStruct {
        fn new(value: usize) -> Self {
            MyStruct {
                inner: AtomicUsize::new(value),
            }
        }
        
        fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize> {
            self.inner.compare_exchange(0, val.get(), Ordering::Release, Ordering::Acquire)
        }
    }

    #[test]
    fn test_compare_exchange_success() {
        let my_struct = MyStruct::new(0);
        let non_zero_value = NonZeroUsize::new(10).unwrap();
        
        assert_eq!(my_struct.compare_exchange(non_zero_value), Ok(0));
        assert_eq!(my_struct.inner.load(Ordering::Acquire), 10);
    }

    #[test]
    fn test_compare_exchange_failure() {
        let my_struct = MyStruct::new(5);
        let non_zero_value = NonZeroUsize::new(10).unwrap();
        
        assert_eq!(my_struct.compare_exchange(non_zero_value), Err(5));
        assert_eq!(my_struct.inner.load(Ordering::Acquire), 5);
    }
    
    #[test]
    #[should_panic]
    fn test_compare_exchange_with_zero() {
        let my_struct = MyStruct::new(0);
        let zero_value = NonZeroUsize::new(0).expect("should not be zero");
        my_struct.compare_exchange(zero_value);
    }
}

