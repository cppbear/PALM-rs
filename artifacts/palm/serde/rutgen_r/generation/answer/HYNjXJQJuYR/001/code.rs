// Answer 0

#[test]
fn test_visit_string_success() {
    use std::ffi::CString;
    use serde::de::Error;

    struct TestVisitor;

    impl serde::de::Visitor for TestVisitor {
        type Value = CString;

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_string("hello".to_string());
    assert!(result.is_ok());
    let c_string = result.unwrap();
    assert_eq!(c_string.to_str().unwrap(), "hello");
}

#[test]
#[should_panic]
fn test_visit_string_panics_on_null_byte() {
    use std::ffi::CString;
    use serde::de::Error;

    struct TestVisitor;

    impl serde::de::Visitor for TestVisitor {
        type Value = CString;

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_string("hello\0world".to_string()); // This should panic
}

#[test]
fn test_visit_string_empty_string() {
    use std::ffi::CString;
    use serde::de::Error;

    struct TestVisitor;

    impl serde::de::Visitor for TestVisitor {
        type Value = CString;

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_string("".to_string());
    assert!(result.is_ok());
    let c_string = result.unwrap();
    assert_eq!(c_string.to_str().unwrap(), "");
}

