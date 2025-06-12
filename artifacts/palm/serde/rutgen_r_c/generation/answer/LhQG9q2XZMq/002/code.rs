// Answer 0

#[test]
fn test_visit_byte_buf_valid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 byte buffer")
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(e) => Err(Error::invalid_value(
                    Unexpected::Bytes(&e.into_bytes()),
                    &self,
                )),
            }
        }
    }

    let visitor = TestVisitor;
    let valid_utf8_bytes: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello" in UTF-8
    assert_eq!(visitor.visit_byte_buf(valid_utf8_bytes).unwrap(), "Hello");
}

#[test]
fn test_visit_byte_buf_invalid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid UTF-8 byte buffer")
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(e) => Err(Error::invalid_value(
                    Unexpected::Bytes(&e.into_bytes()),
                    &self,
                )),
            }
        }
    }

    let visitor = TestVisitor;
    let invalid_utf8_bytes: Vec<u8> = vec![0, 159, 146, 150]; // Invalid UTF-8 bytes
    let result = visitor.visit_byte_buf(invalid_utf8_bytes);
    assert!(result.is_err());
}

