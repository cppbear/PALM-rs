// Answer 0

#[test]
fn test_init_function_with_ok_case_and_compare_exchange_failure() {
    struct TestStruct<'a> {
        value: Option<&'a i32>,
    }
    
    impl<'a> TestStruct<'a> {
        fn compare_exchange(&self, new_value: &'a i32) -> Result<(), &'a i32> {
            if self.value.is_some() {
                Err(self.value.unwrap())
            } else {
                Ok(())
            }
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<&'a i32, E>) -> Result<&'a i32, E> {
            let mut value: &'a i32 = f()?;
            if let Err(old) = self.compare_exchange(value) {
                value = unsafe { &*old };
            }
            Ok(value)
        }
    }

    let initial_value = 1;
    let test_struct = TestStruct { value: Some(&initial_value) };

    let result = test_struct.init(|| Ok(&2));
    assert_eq!(result, Ok(&1));
}

#[test]
fn test_init_function_with_ok_case_and_compare_exchange_failure_boundary() {
    struct TestStruct<'a> {
        value: Option<&'a i32>,
    }

    impl<'a> TestStruct<'a> {
        fn compare_exchange(&self, new_value: &'a i32) -> Result<(), &'a i32> {
            if self.value.is_some() {
                Err(self.value.unwrap())
            } else {
                Ok(())
            }
        }

        fn init<E>(&self, f: impl FnOnce() -> Result<&'a i32, E>) -> Result<&'a i32, E> {
            let mut value: &'a i32 = f()?;
            if let Err(old) = self.compare_exchange(value) {
                value = unsafe { &*old };
            }
            Ok(value)
        }
    }

    let initial_value = 0;
    let test_struct = TestStruct { value: Some(&initial_value) };

    let result = test_struct.init(|| Ok(&1));
    assert_eq!(result, Ok(&0));
}

