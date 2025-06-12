// Answer 0

#[test]
fn test_serialize_integer() {
    let value: i32 = 42;
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[test]
fn test_serialize_string() {
    let value: String = "test".to_string();
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[test]
fn test_serialize_float() {
    let value: f64 = 3.14159;
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[test]
fn test_serialize_nil() {
    let value: Option<i32> = None;
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[test]
fn test_serialize_large_collection() {
    let value: Vec<i32> = (1..10000).collect();
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[test]
fn test_serialize_empty_string() {
    let value: String = "".to_string();
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

#[should_panic]
fn test_serialize_panicking_type() {
    struct NonSerializable;
    let value = NonSerializable;
    let mut serializer = Serializer { writer: vec![], formatter: CompactFormatter };
    serializer.serialize_field(&value);
}

