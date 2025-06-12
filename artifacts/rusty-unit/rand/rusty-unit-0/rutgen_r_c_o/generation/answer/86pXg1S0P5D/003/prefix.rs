// Answer 0

#[test]
fn test_from_ratio_numerator_zero_denominator_zero() {
    let result = from_ratio(0, 0);
}

#[test]
fn test_from_ratio_numerator_equal_denominator() {
    let result = from_ratio(1, 1);
}

#[test]
fn test_from_ratio_numerator_and_denominator_are_zero() {
    let result = from_ratio(0, 1);
}

#[test]
fn test_from_ratio_numerator_is_equal_to_nonzero_denominator() {
    let result = from_ratio(2, 2);
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let result = from_ratio(3, 2);
}

#[test]
fn test_from_ratio_numerator_zero_with_nonzero_denominator() {
    let result = from_ratio(0, 3);
}

#[test]
fn test_from_ratio_numerator_less_than_denominator() {
    let result = from_ratio(1, 2);
}

