// Answer 0

#[test]
fn test_string_deserializer_into_deserializer() {
    use crate::de::IntoDeserializer;

    struct MyError;

    impl de::Error for MyError {
        fn custom<T>(_: T) -> Self {
            MyError
        }
    }
    
    let deserializer = StringDeserializer::<MyError> {
        value: String::from("test"),
        marker: PhantomData,
    };

    let deserializer_out = deserializer.into_deserializer();

    assert_eq!(deserializer_out.value, "test");
}

#[test]
fn test_empty_string_deserializer_into_deserializer() {
    use crate::de::IntoDeserializer;

    struct MyError;

    impl de::Error for MyError {
        fn custom<T>(_: T) -> Self {
            MyError
        }
    }
    
    let deserializer = StringDeserializer::<MyError> {
        value: String::from(""),
        marker: PhantomData,
    };

    let deserializer_out = deserializer.into_deserializer();

    assert_eq!(deserializer_out.value, "");
}

