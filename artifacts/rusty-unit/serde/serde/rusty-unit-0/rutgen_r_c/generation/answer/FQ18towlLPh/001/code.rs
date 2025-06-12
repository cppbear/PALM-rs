// Answer 0

#[test]
fn test_serialize_struct_variant_valid() {
    struct TestError;
    impl std::fmt::Display for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let name = "TestStruct";
    let variant_index = 0;
    let variant = "TestVariant";
    let len = 2;

    let result = serializer.serialize_struct_variant(name, variant_index, variant, len);

    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, name);
    assert_eq!(serialized.variant_index, variant_index);
    assert_eq!(serialized.variant, variant);
    assert_eq!(serialized.fields.capacity(), len);
}

#[test]
fn test_serialize_struct_variant_zero_length() {
    struct TestError;
    impl std::fmt::Display for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for TestError {}

    let serializer = ContentSerializer::<TestError> {
        error: PhantomData,
    };

    let name = "EmptyStruct";
    let variant_index = 1;
    let variant = "EmptyVariant";
    let len = 0;

    let result = serializer.serialize_struct_variant(name, variant_index, variant, len);

    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, name);
    assert_eq!(serialized.variant_index, variant_index);
    assert_eq!(serialized.variant, variant);
    assert_eq!(serialized.fields.capacity(), len);
}

