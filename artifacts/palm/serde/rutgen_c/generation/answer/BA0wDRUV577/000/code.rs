// Answer 0

#[test]
fn test_visit_i32() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a test visitor")
        }

        fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F> 
        where 
            F: de::Error {
            ContentVisitor::new().visit_i32(value).map(TagOrContent::Content)
        }
    }

    let visitor = TestVisitor;
    let result: Result<TagOrContent, TestError> = visitor.visit_i32(42);
    assert!(result.is_ok());
}

#[test]
fn test_visit_i32_invalid() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = TagOrContent<'de>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an invalid visitor")
        }

        fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F> 
        where 
            F: de::Error {
            Err(F::custom("Invalid i32 value"))
        }
    }

    let visitor = InvalidVisitor;
    let result: Result<TagOrContent, TestError> = visitor.visit_i32(42);
    assert!(result.is_err());
}

