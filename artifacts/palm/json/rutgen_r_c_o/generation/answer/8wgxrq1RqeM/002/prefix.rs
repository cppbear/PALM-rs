// Answer 0

#[test]
fn test_serialize_negint_minus10() {
    let number = Number { n: N::NegInt(-10) };
    let serializer = serde_json::Serializer::new();
    let _result = number.serialize(serializer);
}

#[test]
fn test_serialize_negint_minus5() {
    let number = Number { n: N::NegInt(-5) };
    let serializer = serde_json::Serializer::new();
    let _result = number.serialize(serializer);
}

#[test]
fn test_serialize_negint_minus1() {
    let number = Number { n: N::NegInt(-1) };
    let serializer = serde_json::Serializer::new();
    let _result = number.serialize(serializer);
}

#[test]
fn test_serialize_negint_minus100() {
    let number = Number { n: N::NegInt(-100) };
    let serializer = serde_json::Serializer::new();
    let _result = number.serialize(serializer);
}

#[test]
fn test_serialize_negint_minus50() {
    let number = Number { n: N::NegInt(-50) };
    let serializer = serde_json::Serializer::new();
    let _result = number.serialize(serializer);
}

