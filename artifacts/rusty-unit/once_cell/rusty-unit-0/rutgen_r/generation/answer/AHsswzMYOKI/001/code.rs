// Answer 0

#[test]
fn test_init_with_err() {
    use std::num::NonZeroUsize;
    
    struct TestStruct {
        value: Option<NonZeroUsize>
    }

    impl TestStruct {
        fn compare_exchange(&self, _: NonZeroUsize) -> Result<NonZeroUsize, NonZeroUsize> {
            Err(self.value.unwrap())
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
            let nz = f()?;
            let mut val = nz.get();
            if let Err(old) = self.compare_exchange(nz) {
                val = old.get();
            }
            Ok(unsafe { NonZeroUsize::new_unchecked(val) })
        }
    }

    let test_struct = TestStruct { value: Some(unsafe { NonZeroUsize::new_unchecked(5) }) };

    let result: Result<NonZeroUsize, &str> = test_struct.init(|| Err("error"));
    assert_eq!(result, Err("error"));
}

#[test]
#[should_panic]
fn test_init_with_none() {
    use std::num::NonZeroUsize;

    struct TestStruct {
        value: Option<NonZeroUsize>
    }

    impl TestStruct {
        fn compare_exchange(&self, _: NonZeroUsize) -> Result<NonZeroUsize, NonZeroUsize> {
            Err(self.value.unwrap())
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
            let nz = f()?;
            let mut val = nz.get();
            if let Err(old) = self.compare_exchange(nz) {
                val = old.get();
            }
            Ok(unsafe { NonZeroUsize::new_unchecked(val) })
        }
    }

    let test_struct = TestStruct { value: None };

    let _ = test_struct.init(|| Ok(unsafe { NonZeroUsize::new_unchecked(0) }));
}

