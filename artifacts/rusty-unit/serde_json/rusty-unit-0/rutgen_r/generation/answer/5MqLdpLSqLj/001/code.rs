// Answer 0

#[derive(serde::Serialize)]
struct NewtypeStruct {
    field: i32,
}

#[test]
fn test_serialize_newtype_struct_success() {
    let value = NewtypeStruct { field: 42 };
    let result = serialize_newtype_struct("NewtypeStruct", &value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"field":42}"#);
}

#[derive(serde::Serialize)]
struct EmptyStruct;

#[test]
fn test_serialize_newtype_struct_empty() {
    let value = EmptyStruct;
    let result = serialize_newtype_struct("EmptyStruct", &value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{}"#);
}

#[derive(serde::Serialize)]
struct NewtypeWithOption {
    field: Option<i32>,
}

#[test]
fn test_serialize_newtype_struct_option() {
    let value = NewtypeWithOption { field: None };
    let result = serialize_newtype_struct("NewtypeWithOption", &value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"field":null}"#);
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_panic() {
    let value: Option<i32> = None;
    let _ = serialize_newtype_struct("PanicTest", &value).unwrap(); // Should panic due to Option being None.
}

