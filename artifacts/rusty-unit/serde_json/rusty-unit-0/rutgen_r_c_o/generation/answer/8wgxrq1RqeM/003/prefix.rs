// Answer 0

#[test]
fn test_serialize_positive_int_0() {
    let number = Number { n: N::PosInt(0) };
    let _ = serde_json::to_string(&number);
}

#[test]
fn test_serialize_positive_int_1() {
    let number = Number { n: N::PosInt(1) };
    let _ = serde_json::to_string(&number);
}

#[test]
fn test_serialize_positive_int_12345() {
    let number = Number { n: N::PosInt(12345) };
    let _ = serde_json::to_string(&number);
}

#[test]
fn test_serialize_positive_int_max() {
    let number = Number { n: N::PosInt(u64::MAX) };
    let _ = serde_json::to_string(&number);
}

