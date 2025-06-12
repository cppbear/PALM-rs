// Answer 0

#[test]
fn test_from_i128_below_i64_min() {
    let result = Number::from_i128(-170141183460469231731687303715884105728);
}

#[test]
fn test_from_i128_below_i64_min_bound() {
    let result = Number::from_i128(-9223372036854775809);
}

#[test]
fn test_from_i128_above_u64_max() {
    let result = Number::from_i128(9223372036854775808);
}

#[test]
fn test_from_i128_above_u64_max_bound() {
    let result = Number::from_i128(170141183460469231731687303715884105727);
}

