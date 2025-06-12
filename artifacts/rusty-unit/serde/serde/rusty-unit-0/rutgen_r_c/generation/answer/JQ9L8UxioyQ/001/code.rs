// Answer 0

#[test]
fn test_content_ref_deserializer_new_with_bool() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer::new(&content);
    assert!(matches!(deserializer.content, Content::Bool(true)));
}

#[test]
fn test_content_ref_deserializer_new_with_u8() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let content = Content::U8(42);
    let deserializer = ContentRefDeserializer::new(&content);
    assert!(matches!(deserializer.content, Content::U8(42)));
}

#[test]
fn test_content_ref_deserializer_new_with_string() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let content = Content::String("Hello".to_string());
    let deserializer = ContentRefDeserializer::new(&content);
    assert!(matches!(deserializer.content, Content::String(ref s) if s == "Hello"));
}

#[test]
fn test_content_ref_deserializer_new_with_seq() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = ContentRefDeserializer::new(&content);
    if let Content::Seq(ref seq) = *deserializer.content {
        assert_eq!(seq.len(), 2);
        assert!(matches!(seq[0], Content::U8(1)));
        assert!(matches!(seq[1], Content::U8(2)));
    } else {
        panic!("Expected Content::Seq");
    }
}

#[test]
fn test_content_ref_deserializer_new_with_map() {
    struct TestError;
    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display { TestError }
    }

    let content = Content::Map(vec![(Content::String("key".to_string()), Content::U32(123))]);
    let deserializer = ContentRefDeserializer::new(&content);
    if let Content::Map(ref map) = *deserializer.content {
        assert_eq!(map.len(), 1);
        assert!(matches!(map[0].0, Content::String(ref s) if s == "key"));
        assert!(matches!(map[0].1, Content::U32(123)));
    } else {
        panic!("Expected Content::Map");
    }
}

