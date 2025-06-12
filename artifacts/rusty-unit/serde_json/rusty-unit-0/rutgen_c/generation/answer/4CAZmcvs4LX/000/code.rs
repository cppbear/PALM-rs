// Answer 0

#[test]
fn test_serialize_element_bool() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &true;
    let result = serializer.serialize_element(value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    if let Value::Bool(b) = &serializer.vec[0] {
        assert!(*b);
    } else {
        panic!("Expected a Bool value.");
    }
}

#[test]
fn test_serialize_element_number() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &12.5;
    let result = serializer.serialize_element(value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    if let Value::Number(_) = &serializer.vec[0] {
        // Additional checks can be done here for specific number value representation.
    } else {
        panic!("Expected a Number value.");
    }
}

#[test]
fn test_serialize_element_string() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value = &String::from("test");
    let result = serializer.serialize_element(value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    if let Value::String(s) = &serializer.vec[0] {
        assert_eq!(s, "test");
    } else {
        panic!("Expected a String value.");
    }
}

#[test]
fn test_serialize_element_null() {
    let mut serializer = SerializeVec { vec: Vec::new() };
    let value: Option<&str> = None;
    let result = serializer.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(serializer.vec.len(), 1);
    if let Value::Null = &serializer.vec[0] {
        // Null is expected.
    } else {
        panic!("Expected a Null value.");
    }
}

