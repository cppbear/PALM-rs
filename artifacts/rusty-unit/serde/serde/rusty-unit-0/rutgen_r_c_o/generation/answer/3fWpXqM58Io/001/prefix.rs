// Answer 0

#[test]
fn test_serialize_i64_negative_min() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i64(-9223372036854775808);
}

#[test]
fn test_serialize_i64_negative_example() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i64(-1);
}

#[test]
fn test_serialize_i64_zero() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_positive_example() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i64(1);
}

#[test]
fn test_serialize_i64_positive_max() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_i64(9223372036854775807);
}

