// Answer 0

#[test]
fn test_serialize_struct_variant() {
    struct TestError;

    impl ser::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };

    let result = serializer.serialize_struct_variant("TestStruct", 1, "TestVariant", 2);
    assert!(result.is_ok());

    let serialized = result.unwrap();
    assert_eq!(serialized.name, "TestStruct");
    assert_eq!(serialized.variant_index, 1);
    assert_eq!(serialized.variant, "TestVariant");
    assert_eq!(serialized.fields.capacity(), 2);
}

#[test]
fn test_serialize_struct_variant_zero_length() {
    struct TestError;

    impl ser::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    let serializer = ContentSerializer::<TestError> {
        error: std::marker::PhantomData,
    };

    let result = serializer.serialize_struct_variant("TestStruct", 0, "TestVariant", 0);
    assert!(result.is_ok());

    let serialized = result.unwrap();
    assert_eq!(serialized.name, "TestStruct");
    assert_eq!(serialized.variant_index, 0);
    assert_eq!(serialized.variant, "TestVariant");
    assert_eq!(serialized.fields.capacity(), 0);
}

