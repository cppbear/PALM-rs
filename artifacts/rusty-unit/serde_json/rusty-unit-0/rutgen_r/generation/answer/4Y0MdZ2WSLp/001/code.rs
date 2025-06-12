// Answer 0

#[test]
fn test_serialize_newtype_struct_with_string() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    #[derive(Serialize)]
    struct NewtypeStruct(String);

    let value = NewtypeStruct(String::from("Hello world"));
    let serializer = Serializer::new(Vec::new());
    let result: Result<Value> = serializer.serialize_newtype_struct("NewtypeStruct", &value);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, Value::String("Hello world".to_string()));
}

#[test]
fn test_serialize_newtype_struct_with_integer() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    #[derive(Serialize)]
    struct NewtypeStruct(i32);

    let value = NewtypeStruct(42);
    let serializer = Serializer::new(Vec::new());
    let result: Result<Value> = serializer.serialize_newtype_struct("NewtypeStruct", &value);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, Value::Number(42.into()));
}

#[test]
fn test_serialize_newtype_struct_with_empty_struct() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    #[derive(Serialize)]
    struct NewtypeStruct { data: i32 }

    let value = NewtypeStruct { data: 0 };
    let serializer = Serializer::new(Vec::new());
    let result: Result<Value> = serializer.serialize_newtype_struct("NewtypeStruct", &value);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, Value::Object(serde_json::Map::from([(String::from("data"), Value::Number(0.into()))])));
}

#[test]
#[should_panic]
fn test_serialize_newtype_struct_should_panic_on_unserializable() {
    use serde::Serialize;
    use serde_json::ser::Serializer;

    struct UnserializableStruct;

    let value = UnserializableStruct;
    let serializer = Serializer::new(Vec::new());
    let _result: Result<Value> = serializer.serialize_newtype_struct("UnserializableStruct", &value);
}

#[test]
fn test_serialize_newtype_struct_with_floating_point() {
    use serde::Serialize;
    use serde_json::Value;
    use serde_json::ser::Serializer;

    #[derive(Serialize)]
    struct NewtypeStruct(f64);

    let value = NewtypeStruct(3.14);
    let serializer = Serializer::new(Vec::new());
    let result: Result<Value> = serializer.serialize_newtype_struct("NewtypeStruct", &value);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, Value::Number(3.14.into()));
}

