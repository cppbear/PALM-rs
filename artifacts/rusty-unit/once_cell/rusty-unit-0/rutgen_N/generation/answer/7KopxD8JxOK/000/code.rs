// Answer 0

#[test]
fn test_fmt_with_some_value() {
    struct OnceCellTest<T> {
        value: Option<T>,
    }

    impl<T> OnceCellTest<T> {
        fn new(value: T) -> Self {
            OnceCellTest {
                value: Some(value),
            }
        }

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }
    }

    let cell = OnceCellTest::new(42);
    let mut output = String::new();

    let result = writeln!(&mut output, "{:?}", cell);
    assert!(result.is_ok());
    assert_eq!(output.trim(), "OnceCell(42)");
}

#[test]
fn test_fmt_with_none_value() {
    struct OnceCellTest<T> {
        value: Option<T>,
    }

    impl<T> OnceCellTest<T> {
        fn new() -> Self {
            OnceCellTest { value: None }
        }

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }
    }

    let cell: OnceCellTest<i32> = OnceCellTest::new();
    let mut output = String::new();

    let result = writeln!(&mut output, "{:?}", cell);
    assert!(result.is_ok());
    assert_eq!(output.trim(), "OnceCell(Uninit)");
}

