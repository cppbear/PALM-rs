// Answer 0

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let result = from_ratio(5, 3);
    let result = from_ratio(10, 9);
    let result = from_ratio(7, 6);
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator_with_denominator_zero() {
    let result = from_ratio(1, 0);
    let result = from_ratio(3, 0);
    let result = from_ratio(9, 0);
}

