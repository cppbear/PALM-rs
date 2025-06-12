// Answer 0

#[test]
fn test_visit_u16() {
    struct TestVisitor;

    impl de::Visitor for TestVisitor {
        type Value = TagOrContent;
        
        fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(TagOrContent::Content(value))
        }
    }

    struct TestDeserializer;

    impl de::Deserializer for TestDeserializer {
        type Error = TestError;

        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor,
        {
            visitor.visit_u16(42)
        }
    }

    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self 
        where 
            T: std::fmt::Display
        {
            TestError
        }
    }

    let visitor = TestVisitor;
    let result: Result<TagOrContent, TestError> = visitor.visit_u16(10);
    assert_eq!(result, Ok(TagOrContent::Content(10)));

    let result_boundary_low: Result<TagOrContent, TestError> = visitor.visit_u16(0);
    assert_eq!(result_boundary_low, Ok(TagOrContent::Content(0)));

    let result_boundary_high: Result<TagOrContent, TestError> = visitor.visit_u16(u16::MAX);
    assert_eq!(result_boundary_high, Ok(TagOrContent::Content(u16::MAX)));
}

