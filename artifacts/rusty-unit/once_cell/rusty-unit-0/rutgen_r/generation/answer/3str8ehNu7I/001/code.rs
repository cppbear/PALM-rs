// Answer 0

#[test]
fn test_get_initialized_value() {
    struct InnerCell<T>(Option<T>);

    impl<T> InnerCell<T> {
        fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        unsafe fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap()
        }
    }

    struct Cell<T>(InnerCell<T>);

    impl<T> Cell<T> {
        pub fn get(&self) -> Option<&T> {
            if self.0.is_initialized() {
                // Safe because value is initialized.
                Some(unsafe { self.0.get_unchecked() })
            } else {
                None
            }
        }
    }

    let value = 42;
    let inner_cell = InnerCell(Some(value));
    let cell = Cell(inner_cell);
    
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic]
fn test_get_uninitialized_value_panics() {
    struct InnerCell<T>(Option<T>);

    impl<T> InnerCell<T> {
        fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        unsafe fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap()
        }
    }

    struct Cell<T>(InnerCell<T>);

    impl<T> Cell<T> {
        pub fn get(&self) -> Option<&T> {
            if self.0.is_initialized() {
                // Safe because value is initialized.
                Some(unsafe { self.0.get_unchecked() })
            } else {
                None
            }
        }
    }

    let inner_cell = InnerCell::<i32>(None);
    let cell = Cell(inner_cell);
    
    // This will panic because we are trying to access an uninitialized value.
    let _ = cell.get().unwrap();
}

