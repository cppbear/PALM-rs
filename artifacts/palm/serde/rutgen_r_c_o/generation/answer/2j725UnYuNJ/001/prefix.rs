// Answer 0

#[test]
fn test_serialize_f64_zero() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_negative_zero() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(-0.0);
}

#[test]
fn test_serialize_f64_one() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_negative_one() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(-1.0);
}

#[test]
fn test_serialize_f64_min() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(f64::MIN);
}

#[test]
fn test_serialize_f64_max() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(f64::MAX);
}

#[test]
fn test_serialize_f64_epsilon() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(f64::EPSILON);
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_infinity() {
    let mut serializer = FlatMapSerializer::new(); // Initialize with appropriate type.
    let _ = serializer.serialize_f64(f64::INFINITY);
}

