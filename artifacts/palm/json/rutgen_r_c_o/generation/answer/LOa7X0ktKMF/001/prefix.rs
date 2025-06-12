// Answer 0

#[test]
fn test_deserialize_i64_positive() {
    let deserializer = serde_json::from_str("42").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_i64_negative() {
    let deserializer = serde_json::from_str("-42").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_i64_zero() {
    let deserializer = serde_json::from_str("0").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_i128_positive() {
    let deserializer = serde_json::from_str("170141183460469231731687303715884105727").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_i128_negative() {
    let deserializer = serde_json::from_str("-170141183460469231731687303715884105728").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_u64_max() {
    let deserializer = serde_json::from_str("18446744073709551615").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_u64_zero() {
    let deserializer = serde_json::from_str("0").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_u128_max() {
    let deserializer = serde_json::from_str("340282366920938463463374607431768211455").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_f64_max() {
    let deserializer = serde_json::from_str("1.7976931348623157E+308").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_f64_min() {
    let deserializer = serde_json::from_str("-1.7976931348623157E+308").unwrap();
    deserialize(deserializer);
}

#[test]
fn test_deserialize_f64_zero() {
    let deserializer = serde_json::from_str("0.0").unwrap();
    deserialize(deserializer);
}

