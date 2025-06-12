// Answer 0

#[test]
fn test_get_initializes_properly() {
    struct TestCell<T>(OnceCell<T>);

    impl<T> TestCell<T> {
        fn new() -> Self {
            TestCell(OnceCell::new())
        }

        fn initialize(&self, value: T) {
            self.0.set(value).unwrap();
        }

        fn get_value(&self) -> Option<&T> {
            self.0.get()
        }
    }

    let cell: TestCell<i32> = TestCell::new();
    assert_eq!(cell.get_value(), None);
    
    cell.initialize(42);
    assert_eq!(cell.get_value(), Some(&42));
}

#[test]
fn test_get_empty_cell() {
    struct TestCell<T>(OnceCell<T>);

    impl<T> TestCell<T> {
        fn new() -> Self {
            TestCell(OnceCell::new())
        }

        fn get_value(&self) -> Option<&T> {
            self.0.get()
        }
    }

    let cell: TestCell<String> = TestCell::new();
    assert_eq!(cell.get_value(), None);
}

