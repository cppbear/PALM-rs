// Answer 0

#[test]
fn test_visit_string_with_valid_string() {
    use std::ffi::CString;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid C-style string")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_string("Hello".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "Hello");
}

#[test]
fn test_visit_string_with_empty_string() {
    use std::ffi::CString;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid C-style string")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_string("".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "");
}

#[test]
fn test_visit_string_with_invalid_string() {
    use std::ffi::CString;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid C-style string")
        }
    }

    let visitor = TestVisitor;
    let invalid_string = String::from("Invalid\0String");
    let result = visitor.visit_string(invalid_string);

    assert!(result.is_err());
}

