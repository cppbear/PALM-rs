// Answer 0

#[test]
fn test_is_i64_neg_int_values() {
    let negative_one = Number::from(-1);
    let negative_max_i64 = Number::from(i64::MIN);
    let negative_mid_value = Number::from(-5000);
    
    negative_one.is_i64();
    negative_max_i64.is_i64();
    negative_mid_value.is_i64();
}

#[test]
fn test_is_i64_neg_int_min_value() {
    let min_neg_value = Number::from(i64::MIN);
    min_neg_value.is_i64();
}

#[test]
fn test_is_i64_neg_int_values_in_range() {
    let values = [-1, -10, -100, -1000, -10000, -123456789, -2147483648];
    for &value in &values {
        let num = Number::from(value);
        num.is_i64();
    }
}

