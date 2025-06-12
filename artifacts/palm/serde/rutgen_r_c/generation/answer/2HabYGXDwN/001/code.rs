// Answer 0

#[test]
fn test_visit_u64() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("test visitor")
        }
        fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::U64(value))
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(42u64);
    assert_eq!(result, Ok(Content::U64(42u64)));

    let result_max = visitor.visit_u64(u64::MAX);
    assert_eq!(result_max, Ok(Content::U64(u64::MAX)));
}

#[test]
#[should_panic]
fn test_visit_u64_panic() {
    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = Content<'de>;
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("panic visitor")
        }
        fn visit_u64<F>(self, _: u64) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            panic!("This is a forced panic to test panic conditions");
        }
    }

    let panic_visitor = PanicVisitor;
    let _ = panic_visitor.visit_u64(10u64);
}

