// Answer 0

#[test]
fn test_visit_bytes_success() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte sequence")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let byte_array: &[u8] = b"Hello";
    let result = visitor.visit_bytes(byte_array);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_string_lossy(), "Hello");
}

#[test]
fn test_visit_bytes_failure() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte sequence")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let byte_array: &[u8] = b"Hello\0World"; // null byte in between
    let result: Result<CString, _> = visitor.visit_bytes(byte_array);

    assert!(result.is_err());
}

