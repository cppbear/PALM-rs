// Answer 0

#[test]
fn test_visit_borrowed_bytes_valid_utf8() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("valid UTF-8 string")
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = b"Hello, world!";
    let result: Result<&str, _> = visitor.visit_borrowed_bytes(input);
    assert_eq!(result.unwrap(), "Hello, world!");
}

#[test]
#[should_panic]
fn test_visit_borrowed_bytes_invalid_utf8() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("valid UTF-8 string")
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = &[0xFF, 0xFF, 0xFF]; // Invalid UTF-8 sequence
    let _result: Result<&str, _> = visitor.visit_borrowed_bytes(input);
}

#[test]
fn test_visit_borrowed_bytes_empty() {
    struct TestVisitor;

    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("valid UTF-8 string")
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = b""; // Empty input
    let result: Result<&str, _> = visitor.visit_borrowed_bytes(input);
    assert_eq!(result.unwrap(), "");
}

