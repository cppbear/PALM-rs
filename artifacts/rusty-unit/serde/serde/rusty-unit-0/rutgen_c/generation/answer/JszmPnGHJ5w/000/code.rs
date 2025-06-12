// Answer 0

#[test]
fn test_visit_str_valid_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_str("Hello, World!");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_str_invalid_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let invalid_str = "Hello\0World"; // Contains a null byte
    let _ = visitor.visit_str(invalid_str);
}

