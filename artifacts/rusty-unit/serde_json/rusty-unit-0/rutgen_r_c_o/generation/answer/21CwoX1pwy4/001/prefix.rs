// Answer 0

#[test]
fn test_serialize_i64_negative_panic() {
    let writer = Vec::new(); 
    let formatter = CompactFormatter::new(); 
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i64(i64::MIN).unwrap_err();
}

#[test]
fn test_serialize_i64_zero_panic() {
    let writer = Vec::new(); 
    let formatter = CompactFormatter::new(); 
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i64(0).unwrap_err();
}

#[test]
fn test_serialize_i64_positive_panic() {
    let writer = Vec::new(); 
    let formatter = CompactFormatter::new(); 
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i64(i64::MAX).unwrap_err();
}

#[test]
fn test_serialize_i64_negative_middle() {
    let writer = Vec::new(); 
    let formatter = CompactFormatter::new(); 
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i64(-123456789).unwrap_err();
}

#[test]
fn test_serialize_i64_positive_middle() {
    let writer = Vec::new(); 
    let formatter = CompactFormatter::new(); 
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i64(123456789).unwrap_err();
}

