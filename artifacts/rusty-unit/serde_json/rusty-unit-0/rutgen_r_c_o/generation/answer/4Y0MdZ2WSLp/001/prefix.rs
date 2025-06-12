// Answer 0

#[test]
fn test_serialize_newtype_struct_bool_true() {
    let serializer = Serializer;
    let value = &true;
    serializer.serialize_newtype_struct("bool_value", value);
}

#[test]
fn test_serialize_newtype_struct_bool_false() {
    let serializer = Serializer;
    let value = &false;
    serializer.serialize_newtype_struct("bool_value", value);
}

#[test]
fn test_serialize_newtype_struct_u32() {
    let serializer = Serializer;
    let value = &123u32;
    serializer.serialize_newtype_struct("u32_value", value);
}

#[test]
fn test_serialize_newtype_struct_i32() {
    let serializer = Serializer;
    let value = &-123i32;
    serializer.serialize_newtype_struct("i32_value", value);
}

#[test]
fn test_serialize_newtype_struct_u64() {
    let serializer = Serializer;
    let value = &12345678901234567890u64;
    serializer.serialize_newtype_struct("u64_value", value);
}

#[test]
fn test_serialize_newtype_struct_i64() {
    let serializer = Serializer;
    let value = &-1234567890123456789i64;
    serializer.serialize_newtype_struct("i64_value", value);
}

#[test]
fn test_serialize_newtype_struct_u128() {
    let serializer = Serializer;
    let value = &340282366920938463463374607431768211456u128;
    serializer.serialize_newtype_struct("u128_value", value);
}

#[test]
fn test_serialize_newtype_struct_i128() {
    let serializer = Serializer;
    let value = &-170141183460469231731687303715884105728i128;
    serializer.serialize_newtype_struct("i128_value", value);
}

#[test]
fn test_serialize_newtype_struct_string() {
    let serializer = Serializer;
    let value = &String::from("A valid string under 1024 chars");
    serializer.serialize_newtype_struct("string_value", value);
}

#[test]
fn test_serialize_newtype_struct_array() {
    let serializer = Serializer;
    let value = &vec![
        Value::Bool(true),
        Value::Number(Number::from(3.14)),
        Value::String(String::from("another string")),
    ];
    serializer.serialize_newtype_struct("array_value", value);
}

#[test]
fn test_serialize_newtype_struct_object() {
    let serializer = Serializer;
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::Number(Number::from(42)));
    let value = &map;
    serializer.serialize_newtype_struct("object_value", value);
}

