// Answer 0

#[test]
fn test_init_success() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestStruct {
        value: AtomicUsize,
    }

    impl TestStruct {
        fn compare_exchange(&self, new: NonZeroUsize) -> Result<usize, NonZeroUsize> {
            self.value.compare_exchange(
                0,
                new.get(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            ).map_err(|old| unsafe { NonZeroUsize::new_unchecked(old) })
        }
        
        fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
            let nz = f()?;
            let mut val = nz.get();
            if let Err(old) = self.compare_exchange(nz) {
                val = old;
            }
            Ok(unsafe { NonZeroUsize::new_unchecked(val) })
        }
    }

    let test_struct = TestStruct { value: AtomicUsize::new(0) };
    
    let result = test_struct.init(|| Ok(NonZeroUsize::new(10).unwrap()));
    assert_eq!(result.unwrap().get(), 10);
    assert_eq!(test_struct.value.load(Ordering::SeqCst), 10);
}

#[test]
fn test_init_failure_on_exchange() {
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestStruct {
        value: AtomicUsize,
    }

    impl TestStruct {
        fn compare_exchange(&self, new: NonZeroUsize) -> Result<usize, NonZeroUsize> {
            self.value.compare_exchange(
                10,
                new.get(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            ).map_err(|old| unsafe { NonZeroUsize::new_unchecked(old) })
        }
        
        fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
            let nz = f()?;
            let mut val = nz.get();
            if let Err(old) = self.compare_exchange(nz) {
                val = old;
            }
            Ok(unsafe { NonZeroUsize::new_unchecked(val) })
        }
    }

    let test_struct = TestStruct { value: AtomicUsize::new(10) };
    
    let result = test_struct.init(|| Ok(NonZeroUsize::new(20).unwrap()));
    assert_eq!(result.unwrap().get(), 10);
    assert_eq!(test_struct.value.load(Ordering::SeqCst), 10);
}

#[should_panic]
#[test]
fn test_init_invalid_nonzero_failure() {
    use std::num::NonZeroUsize;
    
    struct TestStruct;

    impl TestStruct {
        fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
            let nz = f()?;
            Ok(nz)
        }
    }

    let test_struct = TestStruct;

    // This should panic. Zero is not allowed in NonZeroUsize.
    let _ = test_struct.init(|| Ok(NonZeroUsize::new(0).unwrap()));
}

