// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestError;
    impl de::Error for TestError {}

    let str_deserializer = StrDeserializer {
        value: "test",
        marker: PhantomData::<TestError>,
    };

    let deserializer = str_deserializer.into_deserializer();
    assert_eq!(deserializer.value, "test");
}

