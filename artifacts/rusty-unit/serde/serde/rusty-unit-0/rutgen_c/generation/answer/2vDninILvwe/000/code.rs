// Answer 0

#[test]
fn test_custom_with_string() {
    struct TestDisplay(String);
    
    impl std::fmt::Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let msg = TestDisplay("Test message".to_string());
    let err = fmt::Error::custom(msg);
    assert_eq!(format!("{:?}", err), "fmt::Error");
}

#[test]
fn test_custom_with_integer() {
    struct TestDisplay(i32);
    
    impl std::fmt::Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let msg = TestDisplay(42);
    let err = fmt::Error::custom(msg);
    assert_eq!(format!("{:?}", err), "fmt::Error");
}

#[test]
#[should_panic]
fn test_custom_with_empty_string() {
    struct TestDisplay(String);
    
    impl std::fmt::Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0.is_empty() {
                panic!("Empty message");
            }
            write!(f, "{}", self.0)
        }
    }

    let msg = TestDisplay("".to_string());
    let _err = fmt::Error::custom(msg);
}

