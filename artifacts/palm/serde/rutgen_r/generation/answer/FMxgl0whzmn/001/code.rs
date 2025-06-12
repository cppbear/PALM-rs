// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;
    
    struct TestStruct;

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.expecting(f)
        }
    }

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
    }

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        let result = TestStruct.expecting(&mut formatter);
        assert!(result.is_ok());
    }
    assert_eq!(output, "a string");
}

#[test]
#[should_panic]
fn test_expecting_panic() {
    use std::fmt;

    struct PanicStruct;

    impl fmt::Debug for PanicStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.expecting(f)
        }
    }

    impl PanicStruct {
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("This is a forced panic");
        }
    }

    let panic_instance = PanicStruct;
    let _ = fmt::Debug::fmt(&panic_instance, &mut fmt::Formatter::new(&mut String::new()));
}

