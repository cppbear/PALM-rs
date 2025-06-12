// Answer 0

#[test]
fn test_visit_bytes_valid_utf8() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 encoded byte array")
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = b"Hello, world!";
    let result = visitor.visit_bytes(input);
    assert_eq!(result, Ok("Hello, world!".to_owned()));
}

#[test]
fn test_visit_bytes_empty() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 encoded byte array")
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = b"";
    let result = visitor.visit_bytes(input);
    assert_eq!(result, Ok("".to_owned()));
}

#[test]
fn test_visit_bytes_valid_utf8_with_non_ascii() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 encoded byte array")
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = "こんにちは".as_bytes(); // "Hello" in Japanese
    let result = visitor.visit_bytes(input);
    assert_eq!(result, Ok("こんにちは".to_owned()));
}

