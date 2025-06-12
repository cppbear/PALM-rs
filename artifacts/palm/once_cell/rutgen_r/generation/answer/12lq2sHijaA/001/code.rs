// Answer 0

#[test]
fn test_get_or_init_with_panic_condition() {
    struct MyCell<T> {
        initialized: bool,
        value: Option<T>,
    }

    impl<T> MyCell<T> {
        fn new() -> Self {
            MyCell {
                initialized: false,
                value: None,
            }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> T,
        {
            if self.initialized {
                Ok(self.value.as_ref().unwrap())
            } else {
                let val = f();
                self.value = Some(val);
                self.initialized = true;
                Ok(self.value.as_ref().unwrap())
            }
        }

        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<&T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell: MyCell<i32> = MyCell::new();
    
    // Test that get_or_init initializes the value and returns it.
    let value = cell.get_or_init(|| 42);
    assert_eq!(*value, 42);

    // Test that calling get_or_init again returns the same value without calling f again.
    let same_value = cell.get_or_init(|| 21);
    assert_eq!(*same_value, 42);
}

#[test]
#[should_panic]
fn test_get_or_init_with_panic_when_double_initialization() {
    struct MyCell<T> {
        initialized: bool,
        value: Option<T>,
    }

    impl<T> MyCell<T> {
        fn new() -> Self {
            MyCell {
                initialized: false,
                value: None,
            }
        }

        fn get_or_try_init<F>(&mut self, f: F) -> Result<&T, ()>
        where
            F: FnOnce() -> T,
        {
            if self.initialized {
                Err(())
            } else {
                let val = f();
                self.value = Some(val);
                self.initialized = true;
                Ok(self.value.as_ref().unwrap())
            }
        }

        fn get_or_init<F>(&mut self, f: F) -> &T
        where
            F: FnOnce() -> T,
        {
            enum Void {}
            match self.get_or_try_init(|| Ok::<&T, Void>(f())) {
                Ok(val) => val,
                Err(void) => match void {},
            }
        }
    }

    let mut cell: MyCell<i32> = MyCell::new();
    
    // Initialize the cell
    cell.get_or_init(|| 42);
    
    // This should trigger a panic because we're trying to reinitialize.
    let _ = cell.get_or_init(|| 21);
}

