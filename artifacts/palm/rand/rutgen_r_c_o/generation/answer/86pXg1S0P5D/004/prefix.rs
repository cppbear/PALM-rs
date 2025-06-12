// Answer 0

#[test]
fn test_from_ratio_numerator_zero_denominator_one() {
    let result = from_ratio(0, 1);
}

#[test]
fn test_from_ratio_numerator_one_denominator_one() {
    let result = from_ratio(1, 1);
}

#[test]
#[should_panic]
fn test_from_ratio_numerator_zero_denominator_zero() {
    let result = from_ratio(0, 0);
}

#[test]
#[should_panic]
fn test_from_ratio_numerator_one_denominator_zero() {
    let result = from_ratio(1, 0);
}

