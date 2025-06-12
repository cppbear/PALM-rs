// Answer 0

#[test]
fn test_into_deserializer() {
    struct TestStruct {
        value: i32,
    }

    let instance = TestStruct { value: 10 };
    let result = instance.into_deserializer();
    assert_eq!(result.value, 10);
}

#[test]
fn test_into_deserializer_empty_struct() {
    struct EmptyStruct;

    let instance = EmptyStruct;
    let result = instance.into_deserializer();
    // Since we cannot really check the contents of an empty struct,
    // we will simply ensure the reference equality holds true.
    assert!(std::ptr::eq(&instance, &result));
}

