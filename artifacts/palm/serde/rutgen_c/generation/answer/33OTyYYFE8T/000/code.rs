// Answer 0

#[test]
fn test_serialize_i8() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {}

    let serializer = ContentSerializer::<DummyError> { error: std::marker::PhantomData };
    let result = serializer.serialize_i8(42).unwrap();

    match result {
        Content::I8(val) => assert_eq!(val, 42),
        _ => panic!("Expected Content::I8"),
    }
}

#[test]
fn test_serialize_i8_negative() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {}

    let serializer = ContentSerializer::<DummyError> { error: std::marker::PhantomData };
    let result = serializer.serialize_i8(-1).unwrap();

    match result {
        Content::I8(val) => assert_eq!(val, -1),
        _ => panic!("Expected Content::I8"),
    }
}

