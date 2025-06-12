// Answer 0

#[test]
fn test_try_insert_with_existing_value() {
    struct TestOnceCell {
        inner: std::cell::UnsafeCell<Option<i32>>,
    }

    impl TestOnceCell {
        fn new() -> Self {
            TestOnceCell {
                inner: std::cell::UnsafeCell::new(None),
            }
        }

        fn get(&self) -> Option<&i32> {
            unsafe { &*self.inner.get() }
        }

        fn try_insert(&self, value: i32) -> Result<&i32, (&i32, i32)> {
            if let Some(old) = self.get() {
                return Err((old, value));
            }

            let slot = unsafe { &mut *self.inner.get() };
            *slot = Some(value);
            Ok(unsafe { slot.as_ref().unwrap_unchecked() })
        }
    }

    let cell = TestOnceCell::new();
    
    // First insertion should succeed
    assert!(cell.try_insert(92).is_ok());
    
    // Try inserting again, which should return an error
    let result = cell.try_insert(62);
    assert!(result.is_err());
    // Check that the error contains the value 92 and the new value 62
    let (old_value, new_value) = result.unwrap_err();
    assert_eq!(*old_value, 92);
    assert_eq!(new_value, 62);
}

