// Answer 0

#[test]
fn test_serialize_unit_struct_err() {
    struct UnitStruct;

    let unit_struct = UnitStruct;
    let result: Result<String> = serialize_unit_struct(unit_struct, "UnitStructName");

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, key_must_be_a_string());
    }
}

