// Answer 0

#[test]
fn test_serialize_some_with_bool() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = true;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_integer() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = 42;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_float() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = 3.14;
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_char() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = 'a';
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_array() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = [1, 2, 3];
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_tuple() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = (1, 'b');
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_custom_struct() {
    #[derive(serde::Serialize)]
    struct CustomStruct {
        field1: i32,
        field2: char,
    }

    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = CustomStruct { field1: 24, field2: 'c' };
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_vector() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = vec![1, 2, 3, 4];
    serializer.serialize_some(&value);
}

#[test]
fn test_serialize_some_with_nested() {
    #[derive(serde::Serialize)]
    struct Outer {
        inner: Vec<i32>,
    }
    
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let value = Outer { inner: vec![5, 6, 7] };
    serializer.serialize_some(&value);
}

