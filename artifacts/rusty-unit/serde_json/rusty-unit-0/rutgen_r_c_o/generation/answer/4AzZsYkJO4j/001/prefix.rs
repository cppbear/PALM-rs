// Answer 0

#[test]
fn test_serialize_element_empty_slice() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let empty_slice: Vec<i32> = Vec::new();
    serializer.serialize_element(&empty_slice).unwrap();
}

#[test]
fn test_serialize_element_single_int() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let single_value = 42;
    serializer.serialize_element(&single_value).unwrap();
}

#[test]
fn test_serialize_element_single_string() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let single_value = "test string";
    serializer.serialize_element(&single_value).unwrap();
}

#[test]
fn test_serialize_element_multiple_elements() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let values = vec![1, 2, 3, 4, 5];
    for value in values {
        serializer.serialize_element(&value).unwrap();
    }
}

#[test]
fn test_serialize_element_empty_string() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let empty_value = "";
    serializer.serialize_element(&empty_value).unwrap();
}

#[test]
fn test_serialize_element_complex_object() {
    #[derive(Serialize)]
    struct Complex {
        id: i32,
        name: String,
    }

    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let complex_object = Complex { id: 1, name: String::from("test") };
    serializer.serialize_element(&complex_object).unwrap();
}

#[test]
#[should_panic]
fn test_serialize_element_non_serializable() {
    struct NonSerializable;

    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter };
    let non_serializable_value = NonSerializable;
    serializer.serialize_element(&non_serializable_value);
}

