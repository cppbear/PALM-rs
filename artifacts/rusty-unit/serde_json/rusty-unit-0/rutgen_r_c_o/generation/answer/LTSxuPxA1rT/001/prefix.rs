// Answer 0

#[test]
fn test_serialize_i128_positive_u64() {
    let serializer = Serializer;
    let _ = serializer.serialize_i128(0);
    let _ = serializer.serialize_i128(1);
    let _ = serializer.serialize_i128(2_i128.pow(63) - 1);
}

#[test]
fn test_serialize_i128_positive_i64() {
    let serializer = Serializer;
    let _ = serializer.serialize_i128(-2_i128.pow(63));
    let _ = serializer.serialize_i128(123456);
    let _ = serializer.serialize_i128(2_i128.pow(62));
}

#[test]
fn test_serialize_i128_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i128(-2_i128.pow(63) - 1);
    let _ = result; // should trigger Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = Serializer;
    let _ = serializer.serialize_i128(0);
}

#[test]
fn test_serialize_i128_overflow() {
    let serializer = Serializer;
    let result = serializer.serialize_i128(2_i128.pow(63));
    let _ = result; // should trigger Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
}

#[test]
fn test_serialize_i128_max_i64() {
    let serializer = Serializer;
    let _ = serializer.serialize_i128(2_i128.pow(63) - 1);
}

#[test]
fn test_serialize_i128_min_i64() {
    let serializer = Serializer;
    let _ = serializer.serialize_i128(-2_i128.pow(63));
}

