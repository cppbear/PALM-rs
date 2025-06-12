// Answer 0

#[test]
fn test_serialize_f64_negative_infinity() {
    let serializer = MapKeySerializer;
    let value = -std::f64::INFINITY;
    let _result = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_positive_infinity() {
    let serializer = MapKeySerializer;
    let value = std::f64::INFINITY;
    let _result = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = MapKeySerializer;
    let value = std::f64::NAN;
    let _result = serializer.serialize_f64(value);
}

