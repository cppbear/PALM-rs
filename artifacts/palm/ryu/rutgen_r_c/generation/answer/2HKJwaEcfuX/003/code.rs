// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_negative_input() {
    let input = -1;
    let _ = log10_pow2(input);
}

#[test]
fn test_log10_pow2_zero_input() {
    let input = 0;
    let expected_output = 0; // since (0 * 78913) >> 18 = 0
    assert_eq!(log10_pow2(input), expected_output);
}

#[test]
fn test_log10_pow2_upper_bound() {
    let input = 1650;
    let expected_output = 78913 * 1650 >> 18; // expected calculation for the maximum valid input
    assert_eq!(log10_pow2(input), expected_output);
} 

#[test]
#[should_panic]
fn test_log10_pow2_exceeding_upper_bound() {
    let input = 1651;
    let _ = log10_pow2(input);
}

