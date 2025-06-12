// Answer 0

#[test]
fn test_visit_borrowed_str_valid() {
    struct TestVisitor;
    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed string")
        }
    }

    let visitor = TestVisitor;
    let input = "Hello, world!";
    let result = visitor.visit_borrowed_str(input);
    assert_eq!(result.unwrap(), input);
}

#[test]
#[should_panic]
fn test_visit_borrowed_str_invalid() {
    struct TestVisitor;
    impl<'a> Visitor<'a> for TestVisitor {
        type Value = &'a str;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a borrowed string")
        }
    }

    let visitor = TestVisitor;
    let input: &[u8] = &[0xFF]; // Invalid UTF-8
    visitor.visit_borrowed_bytes(input).unwrap(); // This should panic
}

