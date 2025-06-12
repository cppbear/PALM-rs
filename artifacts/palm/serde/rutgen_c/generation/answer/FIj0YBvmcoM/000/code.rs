// Answer 0

#[test]
fn test_visit_borrowed_bytes_success() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a [u8];
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte slice")
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = b"test bytes";

    let result = visitor.visit_borrowed_bytes(input);
    assert_eq!(result.unwrap(), input);
}

#[test]
fn test_visit_borrowed_str_conversion() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a [u8];
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte slice")
        }
    }

    let visitor = TestVisitor;
    let input: &str = "test string";

    let result = visitor.visit_borrowed_str(input);
    assert_eq!(result.unwrap(), input.as_bytes());
}

