// Answer 0

#[test]
fn test_visit_string_valid() {
    use std::ffi::CString;
    use serde::de::{Error, Visitor};

    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Value = CString;

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_string("valid string".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "valid string");
}

#[test]
#[should_panic]
fn test_visit_string_invalid() {
    use std::ffi::CString;
    use serde::de::{Error, Visitor};

    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Value = CString;

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_string("invalid\0string".to_string());
}

