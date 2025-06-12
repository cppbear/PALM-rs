// Answer 0

#[test]
fn test_visit_borrowed_bytes_valid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }
    }

    let visitor = TestVisitor;
    let bytes: &[u8] = b"valid";
    let result = visitor.visit_borrowed_bytes(bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "valid");
}

#[test]
fn test_visit_borrowed_bytes_invalid_utf8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid UTF-8 string")
        }
    }

    let visitor = TestVisitor;
    let bytes: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let result = visitor.visit_borrowed_bytes(bytes);
    assert!(result.is_err());
}

