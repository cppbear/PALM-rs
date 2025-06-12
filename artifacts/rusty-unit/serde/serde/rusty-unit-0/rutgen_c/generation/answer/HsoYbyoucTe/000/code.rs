// Answer 0

#[test]
fn test_expecting() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let visitor = TestVisitor;
    let mut output = String::new();
    let result = visitor.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "test");
}

#[test]
fn test_expecting_default() {
    let ignored_any = IgnoredAny::default();
    let mut output = String::new();
    let result = ignored_any.expecting(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "anything at all");
}

