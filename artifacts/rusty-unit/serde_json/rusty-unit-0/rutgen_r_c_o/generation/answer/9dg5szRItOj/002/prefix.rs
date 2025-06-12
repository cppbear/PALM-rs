// Answer 0

#[test]
fn test_from_i128_valid_u64() {
    let result = Number::from_i128(0);
}

#[test]
fn test_from_i128_valid_u64_bound() {
    let result = Number::from_i128(18446744073709551615);
}

#[test]
fn test_from_i128_valid_i64() {
    let result = Number::from_i128(-1);
}

#[test]
fn test_from_i128_valid_i64_bound() {
    let result = Number::from_i128(-9223372036854775808);
}

#[test]
fn test_from_i128_out_of_range_neg() {
    let result = Number::from_i128(-9223372036854775809);
}

#[test]
fn test_from_i128_out_of_range_pos() {
    let result = Number::from_i128(18446744073709551616);
}

#[test]
fn test_from_i128_arbitrary_precision() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let result = Number::from_i128(170141183460469231731687303715884105728);
    }
}

#[test]
fn test_from_i128_arbitrary_precision_neg() {
    #[cfg(feature = "arbitrary_precision")]
    {
        let result = Number::from_i128(-170141183460469231731687303715884105729);
    }
}

