// Answer 0

#[derive(Serialize)]
struct NewTypeStruct {
    value: i32,
}

#[test]
fn test_serialize_newtype_struct_valid() {
    let value = NewTypeStruct { value: 42 };
    let result = serialize_newtype_struct("newtype", &value).unwrap();
    assert_eq!(result, "{\"value\":42}");
}

#[test]
fn test_serialize_newtype_struct_empty() {
    let value = NewTypeStruct { value: 0 };
    let result = serialize_newtype_struct("newtype", &value).unwrap();
    assert_eq!(result, "{\"value\":0}");
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_fail() {
    let value = "not a serializable struct"; // string does not implement Serialize
    serialize_newtype_struct("newtype", &value).unwrap();
}

