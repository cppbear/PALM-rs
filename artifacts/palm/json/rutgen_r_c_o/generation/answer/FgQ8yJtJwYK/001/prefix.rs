// Answer 0

#[test]
fn test_serialize_f64_zero() {
    Serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_positive() {
    Serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_negative() {
    Serializer.serialize_f64(-1.0);
}

#[test]
fn test_serialize_f64_large_positive() {
    Serializer.serialize_f64(1e10);
}

#[test]
fn test_serialize_f64_large_negative() {
    Serializer.serialize_f64(-1e10);
}

#[test]
fn test_serialize_f64_infinity() {
    Serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_neg_infinity() {
    Serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_nan() {
    Serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_min() {
    Serializer.serialize_f64(f64::MIN);
}

#[test]
fn test_serialize_f64_max() {
    Serializer.serialize_f64(f64::MAX);
}

