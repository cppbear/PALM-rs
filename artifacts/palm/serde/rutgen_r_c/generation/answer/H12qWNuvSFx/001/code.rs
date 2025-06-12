// Answer 0

#[test]
fn test_serialize_f64_positive() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f64(3.14);
    assert_eq!(result, Ok(Content::F64(3.14)));
}

#[test]
fn test_serialize_f64_negative() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f64(-2.71);
    assert_eq!(result, Ok(Content::F64(-2.71)));
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f64(0.0);
    assert_eq!(result, Ok(Content::F64(0.0)));
}

#[test]
fn test_serialize_f64_large() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f64(1e10);
    assert_eq!(result, Ok(Content::F64(1e10)));
}

#[test]
fn test_serialize_f64_small() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f64(1e-10);
    assert_eq!(result, Ok(Content::F64(1e-10)));
}

