// Answer 0

#[test]
fn test_serialize_f64_finite_positive() {
    let serializer = MapKeySerializer;
    let value = 123.456_f64;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_finite_negative() {
    let serializer = MapKeySerializer;
    let value = -123.456_f64;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_finite_small() {
    let serializer = MapKeySerializer;
    let value = 1.0e-10_f64;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_finite_large() {
    let serializer = MapKeySerializer;
    let value = 1.0e+10_f64;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_finite_max() {
    let serializer = MapKeySerializer;
    let value = 1.7976931348623157e+308_f64;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_finite_min() {
    let serializer = MapKeySerializer;
    let value = -1.7976931348623157e+308_f64;
    let _ = serializer.serialize_f64(value);
}

