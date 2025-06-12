// Answer 0

#[test]
fn test_visit_bytes_valid_utf8() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            std::str::from_utf8(v)
                .map(From::from)
                .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let bytes = b"Hello, world!";
    let result = visitor.visit_bytes(bytes);
    assert_eq!(result.unwrap(), "Hello, world!");
}

#[test]
fn test_visit_bytes_invalid_utf8() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            std::str::from_utf8(v)
                .map(From::from)
                .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let invalid_bytes = &[0, 159, 146, 150];
    let result: Result<String, _> = visitor.visit_bytes(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_visit_bytes_empty() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            std::str::from_utf8(v)
                .map(From::from)
                .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = TestVisitor;
    let bytes: &[u8] = &[];
    let result = visitor.visit_bytes(bytes);
    assert_eq!(result.unwrap(), "");
}

