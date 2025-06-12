// Answer 0

#[test]
fn test_invalid_type() {
    struct MockUnexpected;
    
    impl std::fmt::Display for MockUnexpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "mock unexpected type")
        }
    }

    struct MockExpected;

    impl std::fmt::Display for MockExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "mock expected type")
        }
    }

    let unexp = de::Unexpected::Other("mock");
    let exp: &dyn de::Expected = &MockExpected;
    
    let error = invalid_type(unexp, exp);

    assert_eq!(format!("{}", error), "invalid type: mock unexpected type, expected mock expected type");
}

