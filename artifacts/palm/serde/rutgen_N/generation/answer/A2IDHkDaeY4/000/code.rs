// Answer 0

#[test]
fn test_visit_bytes_valid_utf8() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_bytes<E>(self, v: &[u8]) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            match std::str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let input = b"valid utf8 string";
    let result: Result<String, _> = visitor.visit_bytes(input);
    assert_eq!(result.unwrap(), "valid utf8 string");
}

#[test]
fn test_visit_bytes_invalid_utf8() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_bytes<E>(self, v: &[u8]) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            match std::str::from_utf8(v) {
                Ok(s) => Ok(s.to_owned()),
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = TestVisitor;
    let input = b"\xFF\xFE\xFD"; // Invalid UTF-8
    let result: Result<String, _> = visitor.visit_bytes(input);
    assert!(result.is_err());
}

