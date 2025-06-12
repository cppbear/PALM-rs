// Answer 0

#[test]
fn test_as_i64_pos_int_above_max() {
    let number = Number::from_u64(9223372036854775808).unwrap();
    let _result = number.as_i64();
}

#[test]
fn test_as_i64_pos_int_exceeding_max() {
    let number = Number::from_u64(9223372036854775808_u64).unwrap();
    let _result = number.as_i64();
}

