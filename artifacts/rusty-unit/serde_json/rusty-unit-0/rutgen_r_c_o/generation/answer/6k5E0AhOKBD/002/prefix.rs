// Answer 0

#[test]
fn test_is_u64_with_negative_integer() {
    let number_neg_one = Number::from(-1i64);
    number_neg_one.is_u64();

    let number_neg_hundred = Number::from(-100i64);
    number_neg_hundred.is_u64();

    let number_neg_two_billion = Number::from(i64::MIN);
    number_neg_two_billion.is_u64();

    let number_neg_max = Number::from(-9223372036854775808i64);
    number_neg_max.is_u64();
}

