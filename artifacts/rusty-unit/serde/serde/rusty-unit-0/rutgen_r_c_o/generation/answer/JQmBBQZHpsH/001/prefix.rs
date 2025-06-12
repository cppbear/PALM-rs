// Answer 0

#[test]
fn test_serialize_i32_negative() {
    let serializer = FlatMapSerializer(&mut HashMap::new());
    let result = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = FlatMapSerializer(&mut HashMap::new());
    let result = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive() {
    let serializer = FlatMapSerializer(&mut HashMap::new());
    let result = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_i32_min() {
    let serializer = FlatMapSerializer(&mut HashMap::new());
    let result = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_max() {
    let serializer = FlatMapSerializer(&mut HashMap::new());
    let result = serializer.serialize_i32(2147483647);
}

