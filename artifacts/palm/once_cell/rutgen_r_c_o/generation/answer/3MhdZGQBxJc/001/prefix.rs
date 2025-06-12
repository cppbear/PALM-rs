// Answer 0

#[test]
fn test_once_cell_with_initialized_value() {
    struct TestStruct {
        value: i32,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct({})", self.value)
        }
    }

    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let mut formatter = fmt::Formatter::new();
    cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_with_another_initialized_value() {
    struct AnotherStruct {
        text: String,
    }

    impl fmt::Debug for AnotherStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherStruct({})", self.text)
        }
    }

    let cell = OnceCell::with_value(AnotherStruct { text: String::from("Hello") });
    let mut formatter = fmt::Formatter::new();
    cell.fmt(&mut formatter);
}

#[test]
fn test_once_cell_with_empty_struct() {
    #[derive(Debug)]
    struct EmptyStruct;

    let cell = OnceCell::with_value(EmptyStruct);
    let mut formatter = fmt::Formatter::new();
    cell.fmt(&mut formatter);
}

