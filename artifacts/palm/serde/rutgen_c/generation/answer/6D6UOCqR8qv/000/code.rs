// Answer 0

#[test]
fn test_borrowed_str_deserializer_into_deserializer() {
    struct TestError;

    impl de::Error for TestError {}

    let borrowed_str_deserializer = BorrowedStrDeserializer {
        value: "test",
        marker: PhantomData::<TestError>,
    };

    let deserializer = borrowed_str_deserializer.into_deserializer();

    assert_eq!(deserializer.value, "test");
}

