// Answer 0

#[test]
fn test_serialize_char() {
    struct TestError;

    impl serde::ser::Error for TestError {
        fn custom<T>(msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };

    let character = 'a';
    let result = serializer.serialize_char(character);

    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Char(c) = content {
            assert_eq!(c, character);
        } else {
            panic!("Expected Content::Char, got different variant");
        }
    }
}

#[test]
fn test_serialize_char_empty() {
    struct TestError;

    impl serde::ser::Error for TestError {
        fn custom<T>(msg: T) -> Self {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };

    let character = ' ';
    let result = serializer.serialize_char(character);

    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Char(c) = content {
            assert_eq!(c, character);
        } else {
            panic!("Expected Content::Char, got different variant");
        }
    }
}

