// Answer 0

#[test]
fn test_into_deserializer_bytes() {
    struct DummyError;
    impl serde::de::Error for DummyError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            DummyError
        }
    }

    let value: &[u8] = &[1, 2, 3];
    let deserializer = BytesDeserializer {
        value,
        marker: std::marker::PhantomData::<DummyError>,
    };
    
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, value);
}

#[test]
fn test_into_deserializer_unit() {
    struct DummyError;
    impl serde::de::Error for DummyError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            DummyError
        }
    }

    let deserializer = serde::de::UnitDeserializer {};
    let result = deserializer.into_deserializer();
    assert_eq!(std::mem::size_of_val(&result), std::mem::size_of_val(&deserializer));
}

#[test]
fn test_into_deserializer_bool() {
    struct DummyError;
    impl serde::de::Error for DummyError {
        fn custom<T: std::fmt::Display>(msg: T) -> Self {
            DummyError
        }
    }

    let value = true;
    let deserializer = BoolDeserializer::new(value);
    
    let result = deserializer.into_deserializer();
    assert_eq!(result.value, value);
}

