// Answer 0

#[test]
fn test_serialize_newtype_struct_success() {
    use serde::Serialize;
    use serde_json::serialize_newtype_struct;

    #[derive(Serialize)]
    struct Newtype(String);

    let newtype_instance = Newtype("test".to_string());
    let serialized = serialize_newtype_struct("Newtype", &newtype_instance).unwrap();
    assert_eq!(serialized, r#"{"Newtype":"test"}"#);
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_failure() {
    use serde::Serialize;
    use serde_json::serialize_newtype_struct;

    struct NonSerializable;

    let non_serializable_instance = NonSerializable;
    let _serialized = serialize_newtype_struct("NonSerializable", &non_serializable_instance).unwrap();
}

