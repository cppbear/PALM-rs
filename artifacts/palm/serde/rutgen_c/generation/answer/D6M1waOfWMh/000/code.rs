// Answer 0

#[test]
fn test_visit_borrowed_bytes_valid_utf8() {
    struct TestVisitor;
    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }
        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            str::from_utf8(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let valid_bytes = b"Hello, world!";
    let result = visitor.visit_borrowed_bytes(valid_bytes);
    assert_eq!(result, Ok("Hello, world!"));
}

#[test]
fn test_visit_borrowed_bytes_invalid_utf8() {
    struct TestVisitor;
    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }
        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            str::from_utf8(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let invalid_bytes = &[0, 159, 146, 150]; // Invalid UTF-8 sequence
    let result: Result<&str, _> = visitor.visit_borrowed_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_visit_borrowed_bytes_empty() {
    struct TestVisitor;
    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }
        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            str::from_utf8(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let empty_bytes: &[u8] = b"";
    let result = visitor.visit_borrowed_bytes(empty_bytes);
    assert_eq!(result, Ok(""));
}

