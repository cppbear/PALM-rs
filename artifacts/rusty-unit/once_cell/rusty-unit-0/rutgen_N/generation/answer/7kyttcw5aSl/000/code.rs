// Answer 0

#[test]
fn test_init_success() {
    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn compare_exchange(&mut self, new_value: T) -> Result<*const T, T> {
            if self.value.is_none() {
                self.value = Some(new_value);
                Ok(std::ptr::null())
            } else {
                Err(new_value)
            }
        }

        fn init<E>(&mut self, f: impl FnOnce() -> Result<T, E>) -> Result<&T, E> {
            let mut value: T = f()?;
            if let Err(old) = self.compare_exchange(value) {
                value = unsafe { &*old };
            }
            Ok(&value)
        }
    }

    let mut cell: OnceCell<i32> = OnceCell::new();
    let result = cell.init(|| Ok(42));
    assert_eq!(result.unwrap(), &42);
}

#[test]
fn test_init_existing_value() {
    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn compare_exchange(&mut self, new_value: T) -> Result<*const T, T> {
            if self.value.is_none() {
                self.value = Some(new_value);
                Ok(std::ptr::null())
            } else {
                Err(new_value)
            }
        }

        fn init<E>(&mut self, f: impl FnOnce() -> Result<T, E>) -> Result<&T, E> {
            let mut value: T = f()?;
            if let Err(old) = self.compare_exchange(value) {
                value = unsafe { &*old };
            }
            Ok(&value)
        }
    }

    let mut cell: OnceCell<i32> = OnceCell::new();
    let _ = cell.init(|| Ok(42));
    let result = cell.init(|| Ok(100));
    assert_eq!(result.unwrap(), &42);
}

