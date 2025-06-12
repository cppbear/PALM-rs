// Answer 0

#[test]
fn test_random_ratio_valid_cases() {
    let result_2_3 = random_ratio(2, 3);
    assert!(result_2_3 == true || result_2_3 == false);

    let result_1_1 = random_ratio(1, 1);
    assert_eq!(result_1_1, true);

    let result_0_1 = random_ratio(0, 1);
    assert_eq!(result_0_1, false);
}

#[test]
#[should_panic(expected = "denominator = 0")]
fn test_random_ratio_zero_denominator() {
    random_ratio(1, 0);
}

#[test]
#[should_panic(expected = "numerator > denominator")]
fn test_random_ratio_numerator_greater_than_denominator() {
    random_ratio(3, 2);
}

#[test]
fn test_random_ratio_edge_cases() {
    let result_0_0 = random_ratio(0, 0);
    assert!(result_0_0 == true || result_0_0 == false);
}

