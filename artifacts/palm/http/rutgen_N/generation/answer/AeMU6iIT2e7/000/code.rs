// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn get_ref(&self) -> &String {
            &self.value
        }
    }

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.get_ref(), f)
        }
    }

    let test_instance = TestStruct {
        value: String::from("Hello, World!"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output, "Hello, World!");
}

