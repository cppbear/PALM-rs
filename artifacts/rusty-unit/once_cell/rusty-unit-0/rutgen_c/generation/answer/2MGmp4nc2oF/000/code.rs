// Answer 0

#[test]
fn test_into_inner_empty() {
    struct TestOnceCell(Imp<String>);

    impl OnceCell<String> {
        pub const fn new() -> Self {
            OnceCell(Imp::new())
        }

        pub fn into_inner(self) -> Option<String> {
            self.0.into_inner()
        }
    }

    let cell: OnceCell<String> = OnceCell::new();
    assert_eq!(cell.into_inner(), None);
}

#[test]
fn test_into_inner_with_value() {
    struct TestOnceCell(Imp<String>);

    impl OnceCell<String> {
        pub const fn new() -> Self {
            OnceCell(Imp::new())
        }

        pub fn set(&self, value: String) -> Result<(), String> {
            self.0.set(value) // Assuming there's an imaginary implementation of this
        }

        pub fn into_inner(self) -> Option<String> {
            self.0.into_inner()
        }
    }

    let cell = OnceCell::new();
    cell.set("hello".to_string()).unwrap();
    assert_eq!(cell.into_inner(), Some("hello".to_string()));
}

