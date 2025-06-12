// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let content: content::ContentRefDeserializer<'static, 'static, TestError> = content::ContentRefDeserializer {
        content: &content::Content::None,
        err: std::marker::PhantomData,
    };

    let deserializer = content.into_deserializer();
    assert_eq!(std::ptr::eq(&deserializer, &content), true);
}

