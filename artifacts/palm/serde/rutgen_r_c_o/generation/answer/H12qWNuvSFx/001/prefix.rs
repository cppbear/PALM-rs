// Answer 0

#[test]
fn test_serialize_f64_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(42.0);
}

#[test]
fn test_serialize_f64_negative() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(-42.0);
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_large_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::MAX);
}

#[test]
fn test_serialize_f64_large_negative() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::MIN);
}

#[test]
fn test_serialize_f64_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::NAN);
}

