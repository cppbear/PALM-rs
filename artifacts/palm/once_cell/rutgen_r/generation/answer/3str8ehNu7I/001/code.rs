// Answer 0

#[test]
fn test_get_initialized_value() {
    struct TestCell<T>(Option<T>);

    impl<T> TestCell<T> {
        fn new(value: T) -> Self {
            TestCell(Some(value))
        }

        fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        unsafe fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap()
        }

        fn get(&self) -> Option<&T> {
            if self.is_initialized() {
                Some(unsafe { self.get_unchecked() })
            } else {
                None
            }
        }
    }

    let cell = TestCell::new(42);
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic]
fn test_get_not_initialized_value() {
    struct TestCell<T>(Option<T>);

    impl<T> TestCell<T> {
        fn new() -> Self {
            TestCell(None)
        }

        fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        unsafe fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap() // This will trigger panic.
        }

        fn get(&self) -> Option<&T> {
            if self.is_initialized() {
                Some(unsafe { self.get_unchecked() })
            } else {
                None
            }
        }
    }

    let empty_cell: TestCell<i32> = TestCell::new();
    let _ = empty_cell.get(); // Should not panic, but gets None.
    // Attempting to call get_unchecked directly will panic.
}

