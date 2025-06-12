// Answer 0

#[test]
fn test_get_when_not_initialized() {
    struct TestCell<T>(Option<T>);

    impl<T> TestCell<T> {
        pub fn new() -> Self {
            TestCell(None)
        }

        pub fn is_initialized(&self) -> bool {
            self.0.is_some()
        }

        pub fn get_unchecked(&self) -> &T {
            self.0.as_ref().unwrap()
        }

        pub fn get(&self) -> Option<&T> {
            if self.is_initialized() {
                Some(unsafe { self.get_unchecked() })
            } else {
                None
            }
        }
    }

    let cell: TestCell<i32> = TestCell::new();
    let result = cell.get();
    assert!(result.is_none());
}

