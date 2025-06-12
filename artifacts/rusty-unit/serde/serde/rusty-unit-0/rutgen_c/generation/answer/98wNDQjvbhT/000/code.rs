// Answer 0

#[test]
fn test_visit_string() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input_string = String::from("test input");
    let result = visitor.visit_string(input_string.clone()).unwrap();
    assert_eq!(result, input_string);
}

#[test]
fn test_visit_string_invalid_utf8() {
    struct InvalidUtf8Visitor;
    impl<'de> Visitor<'de> for InvalidUtf8Visitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(Error::invalid_value(Unexpected::Bytes(v), &self))
        }
    }

    let visitor = InvalidUtf8Visitor;
    let invalid_bytes = b"\x80\x81";
    let result = visitor.visit_bytes(invalid_bytes.as_ref());
    assert!(result.is_err());
}

