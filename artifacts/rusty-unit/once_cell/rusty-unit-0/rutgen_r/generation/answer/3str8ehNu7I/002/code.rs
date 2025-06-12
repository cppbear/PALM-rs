// Answer 0

#[test]
fn get_returns_none_when_not_initialized() {
    struct Cell<T>(Option<T>);

    impl<T> Cell<T> {
        pub fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        pub fn get(&self) -> Option<&T> {
            if self.is_initialized() {
                // Safe b/c value is initialized.
                Some(unsafe { self.get_unchecked() })
            } else {
                None
            }
        }

        unsafe fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap()
        }
    }

    // Create a Cell that is not initialized
    let cell: Cell<i32> = Cell(None);
    
    // Assert that get returns None when the cell is not initialized
    assert_eq!(cell.get(), None);
}

