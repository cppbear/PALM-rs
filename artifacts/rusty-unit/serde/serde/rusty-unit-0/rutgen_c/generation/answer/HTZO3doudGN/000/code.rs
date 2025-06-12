// Answer 0

#[test]
fn test_visit_byte_buf_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_byte_buf(b"hello".to_vec());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "hello");
}

#[test]
#[should_panic]
fn test_visit_byte_buf_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = CString;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid C string")
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            CString::new(v).map_err(Error::custom)
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_byte_buf(b"hello\0world".to_vec());
    assert!(result.is_err());
}

