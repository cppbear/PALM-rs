// Answer 0

#[test]
fn test_serialize_u128_within_range() {
    let serializer = Serializer;
    let value: u128 = 0; // minimum value, should succeed
    serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_at_upper_bound() {
    let serializer = Serializer;
    let value: u128 = u64::MAX as u128; // maximum u64 value, should succeed
    serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_exceeding_u64_max() {
    let serializer = Serializer;
    let value: u128 = u64::MAX as u128 + 1; // just above u64 max, should fail
    serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_at_just_below_2_to_the_128() {
    let serializer = Serializer;
    let value: u128 = u128::MAX; // maximum u128 value, should fail
    serializer.serialize_u128(value);
}

#[test]
fn test_serialize_u128_with_high_value() {
    let serializer = Serializer;
    let value: u128 = 2u128.pow(64) + 1; // a value exceeding u64 range, should fail
    serializer.serialize_u128(value);
}

