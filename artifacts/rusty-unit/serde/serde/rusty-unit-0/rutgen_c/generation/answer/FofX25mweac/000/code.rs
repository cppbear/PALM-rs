// Answer 0

#[test]
fn test_str_deserializer_new() {
    struct MockError;
    impl de::Error for MockError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { MockError }
    }

    let value: &str = "test";
    let deserializer = StrDeserializer::new(value);
    
    assert_eq!(deserializer.value, "test");
}

#[test]
fn test_str_deserializer_new_empty() {
    struct MockError;
    impl de::Error for MockError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { MockError }
    }

    let value: &str = "";
    let deserializer = StrDeserializer::new(value);
    
    assert_eq!(deserializer.value, "");
}

