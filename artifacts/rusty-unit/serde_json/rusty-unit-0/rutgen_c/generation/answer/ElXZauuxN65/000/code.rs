// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct TestSerializer(MapKeySerializer);
    
    let serializer = TestSerializer(MapKeySerializer {});
    let result = serializer.0.serialize_unit_struct("TestStruct");

    assert!(result.is_err());
    match result {
        Err(e) => assert_eq!(e, key_must_be_a_string()),
        _ => panic!("Expected an error"),
    }
}

