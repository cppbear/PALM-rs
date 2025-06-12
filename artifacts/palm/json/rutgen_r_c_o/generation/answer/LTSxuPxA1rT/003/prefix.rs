// Answer 0

#[test]
fn test_serialize_i128_negative_overflow() {
    let serializer = Serializer;
    let value: i128 = i128::MIN; // -2^127, should cause overflow and return Err
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_positive_overflow() {
    let serializer = Serializer;
    let value: i128 = i128::MAX; // 2^127, should cause overflow and return Err
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_minus_one() {
    let serializer = Serializer;
    let value: i128 = -1; // in the valid range, expected to convert to Number
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_neg_one_thirty_three() {
    let serializer = Serializer;
    let value: i128 = -1337; // in the valid range, expected to convert to Number
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = Serializer;
    let value: i128 = 0; // in the valid range, expected to convert to Number
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_eight() {
    let serializer = Serializer;
    let value: i128 = 8; // in the valid range, expected to convert to Number
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_large_number() {
    let serializer = Serializer;
    let value: i128 = 1234567890123456789; // in the valid range, expected to convert to Number
    let _result = serializer.serialize_i128(value);
}

