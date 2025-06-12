// Answer 0

#[test]
fn test_random_ratio_all_false_cases() {
    let _result = random_ratio(0, 1); // Guaranteed to be false
    let _result = random_ratio(0, 2); // Guaranteed to be false
    let _result = random_ratio(0, 3); // Guaranteed to be false
    let _result = random_ratio(0, 4); // Guaranteed to be false
}

#[test]
fn test_random_ratio_all_true_cases() {
    let _result = random_ratio(4, 4); // Guaranteed to be true
}

#[test]
fn test_random_ratio_mixed_cases() {
    let _result = random_ratio(1, 2); // Comes out as true with probability 50%
    let _result = random_ratio(2, 3); // Comes out as true with probability ~67%
    let _result = random_ratio(3, 4); // Comes out as true with probability 75%
}

#[should_panic(expected = "denominator == 0")]
#[test]
fn test_random_ratio_zero_denominator() {
    let _result = random_ratio(1, 0);
}

#[should_panic(expected = "numerator > denominator")]
#[test]
fn test_random_ratio_numerator_greater_than_denominator() {
    let _result = random_ratio(2, 1);
}

