// Answer 0

#[test]
fn test_end_with_empty_tuple() {
    struct ErrorMock;

    impl serde::Error for ErrorMock {
        fn custom<T>(_msg: T) -> Self {
            ErrorMock
        }
    }

    let serialize_tuple: SerializeTuple<ErrorMock> = SerializeTuple {
        elements: Vec::new(),
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple.end();
    assert_eq!(result, Ok(Content::Tuple(Vec::new())));
}

#[test]
fn test_end_with_single_element() {
    struct ErrorMock;

    impl serde::Error for ErrorMock {
        fn custom<T>(_msg: T) -> Self {
            ErrorMock
        }
    }

    let mut serialize_tuple = SerializeTuple {
        elements: vec![Content::U8(42)],
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple.end();
    assert_eq!(result, Ok(Content::Tuple(vec![Content::U8(42)])));
}

#[test]
fn test_end_with_multiple_elements() {
    struct ErrorMock;

    impl serde::Error for ErrorMock {
        fn custom<T>(_msg: T) -> Self {
            ErrorMock
        }
    }

    let mut serialize_tuple = SerializeTuple {
        elements: vec![Content::U8(42), Content::String(String::from("test"))],
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple.end();
    assert_eq!(result, Ok(Content::Tuple(vec![Content::U8(42), Content::String(String::from("test"))])));
}

#[test]
fn test_end_with_different_content_types() {
    struct ErrorMock;

    impl serde::Error for ErrorMock {
        fn custom<T>(_msg: T) -> Self {
            ErrorMock
        }
    }

    let mut serialize_tuple = SerializeTuple {
        elements: vec![
            Content::Bool(true),
            Content::I32(-100),
            Content::F64(3.14),
            Content::Str("example"),
        ],
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple.end();
    assert_eq!(result, Ok(Content::Tuple(vec![
        Content::Bool(true),
        Content::I32(-100),
        Content::F64(3.14),
        Content::Str("example"),
    ])));
}

