// Answer 0

#[derive(Debug)]
struct Content {
    value: f64,
}

impl Content {
    fn F64(v: f64) -> Content {
        Content { value: v }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_f64(self, v: f64) -> Result<Content, &'static str> {
        Ok(Content::F64(v))
    }
}

#[test]
fn test_serialize_positive_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(42.0);
    assert_eq!(result, Ok(Content::F64(42.0)));
}

#[test]
fn test_serialize_negative_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(-42.0);
    assert_eq!(result, Ok(Content::F64(-42.0)));
}

#[test]
fn test_serialize_zero_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(0.0);
    assert_eq!(result, Ok(Content::F64(0.0)));
}

#[test]
fn test_serialize_nan_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NAN);
    assert_eq!(result, Ok(Content::F64(f64::NAN)));
}

#[test]
fn test_serialize_infinity_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::INFINITY);
    assert_eq!(result, Ok(Content::F64(f64::INFINITY)));
}

#[test]
fn test_serialize_negative_infinity_f64() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NEG_INFINITY);
    assert_eq!(result, Ok(Content::F64(f64::NEG_INFINITY)));
}

