// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = Serializer;
    serializer.serialize_bytes(&[]); 
}

#[test]
fn test_serialize_bytes_single_zero() {
    let serializer = Serializer;
    serializer.serialize_bytes(&[0]); 
}

#[test]
fn test_serialize_bytes_single_max() {
    let serializer = Serializer;
    serializer.serialize_bytes(&[255]); 
}

#[test]
fn test_serialize_bytes_multiple() {
    let serializer = Serializer;
    serializer.serialize_bytes(&[0, 255, 128]); 
}

