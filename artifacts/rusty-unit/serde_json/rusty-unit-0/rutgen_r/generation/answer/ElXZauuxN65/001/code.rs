// Answer 0

#[test]
fn test_serialize_unit_struct_err() {
    struct UnitStruct;

    impl serde_json::ser::Serializer for UnitStruct {
        // No need to implement methods for this test; we just need the type.
    }

    let result = UnitStruct.serialize_unit_struct("TestStruct");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), key_must_be_a_string());
}

