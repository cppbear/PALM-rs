// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct MockError;

    impl serde::Error for MockError {}

    let serializer = ContentSerializer::<MockError> {
        error: PhantomData,
    };

    let result = serializer.serialize_unit_struct("TestUnitStruct");

    assert!(result.is_ok());
    
    if let Ok(content) = result {
        match content {
            Content::UnitStruct(name) => assert_eq!(name, "TestUnitStruct"),
            _ => panic!("Expected UnitStruct variant"),
        }
    }
}

#[test]
fn test_serialize_unit_struct_empty_string() {
    struct MockError;

    impl serde::Error for MockError {}

    let serializer = ContentSerializer::<MockError> {
        error: PhantomData,
    };

    let result = serializer.serialize_unit_struct("");

    assert!(result.is_ok());

    if let Ok(content) = result {
        match content {
            Content::UnitStruct(name) => assert_eq!(name, ""),
            _ => panic!("Expected UnitStruct variant"),
        }
    }
}

