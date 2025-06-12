// Answer 0

#[test]
fn test_random_ratio_true() {
    let result = random_ratio(2, 3);
    assert!(result == true || result == false); // Should never panic, result should be a bool.
}

#[test]
fn test_random_ratio_false() {
    let result = random_ratio(0, 5);
    assert_eq!(result, false); // 0 in 5 should always return false.
}

#[test]
fn test_random_ratio_equal_numerator_denominator() {
    let result = random_ratio(3, 3);
    assert_eq!(result, true); // numerator equals denominator should always return true.
}

#[test]
#[should_panic]
fn test_random_ratio_denominator_zero() {
    random_ratio(1, 0); // should panic as denominator is zero.
}

#[test]
#[should_panic]
fn test_random_ratio_numerator_greater_than_denominator() {
    random_ratio(4, 3); // should panic as numerator is greater than denominator.
}

