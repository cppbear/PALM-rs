// Answer 0

#[test]
fn test_serialize_float_min() {
    let number = Number { n: N::Float(f64::MIN) };
    let serializer = serde_json::Serializer::new(serde_json::Serializer::with_arena(serde_json::Arena::new()));
    number.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_float_max() {
    let number = Number { n: N::Float(f64::MAX) };
    let serializer = serde_json::Serializer::new(serde_json::Serializer::with_arena(serde_json::Arena::new()));
    number.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_float_zero() {
    let number = Number { n: N::Float(0.0) };
    let serializer = serde_json::Serializer::new(serde_json::Serializer::with_arena(serde_json::Arena::new()));
    number.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_float_negative() {
    let number = Number { n: N::Float(-3.14) };
    let serializer = serde_json::Serializer::new(serde_json::Serializer::with_arena(serde_json::Arena::new()));
    number.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_float_positive() {
    let number = Number { n: N::Float(2.718) };
    let serializer = serde_json::Serializer::new(serde_json::Serializer::with_arena(serde_json::Arena::new()));
    number.serialize(serializer).unwrap();
}

