// Answer 0

#[test]
fn test_byte_offset_initial() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    assert_eq!(0, stream.byte_offset());
}

#[test]
fn test_byte_offset_after_first_deserialization() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    
    stream.next().unwrap();
    assert_eq!(3, stream.byte_offset());
}

#[test]
fn test_byte_offset_after_second_deserialization() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    
    stream.next().unwrap();
    stream.next().unwrap();
    assert_eq!(7, stream.byte_offset());
}

#[test]
fn test_byte_offset_after_error_deserialization() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();

    stream.next().unwrap();
    stream.next().unwrap();
    let result = stream.next();
    
    assert!(result.is_err());
    assert_eq!(8, stream.byte_offset());
}

