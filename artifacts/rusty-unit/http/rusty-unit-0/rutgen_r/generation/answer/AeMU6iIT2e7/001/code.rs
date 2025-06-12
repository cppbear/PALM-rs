// Answer 0

#[test]
fn test_fmt_with_valid_input() {
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
        value: String::from("Hello, world!"),
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(output, "Hello, world!");
}

#[test]
#[should_panic]
fn test_fmt_with_panic_condition() {
    use std::fmt;

    struct PanicStruct;

    impl PanicStruct {
        fn get_ref(&self) -> &String {
            panic!("This will cause a panic");
        }
    }

    impl fmt::Display for PanicStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.get_ref(), f)
        }
    }

    let test_instance = PanicStruct;

    let _ = format!("{}", test_instance); // This should panic
}

