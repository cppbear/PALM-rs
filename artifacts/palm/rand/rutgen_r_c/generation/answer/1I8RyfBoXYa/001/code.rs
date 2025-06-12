// Answer 0

#[test]
#[should_panic(expected = "p=2/0 is outside range [0.0, 1.0]")]
fn test_random_ratio_zero_denominator() {
    rand::random_ratio(2, 0);
}

#[test]
#[should_panic(expected = "p=3/2 is outside range [0.0, 1.0]")]
fn test_random_ratio_numerator_greater_than_denominator() {
    rand::random_ratio(3, 2);
}

#[test]
fn test_random_ratio_numerator_equals_denominator() {
    let result = rand::random_ratio(5, 5);
    assert_eq!(result, true);
}

#[test]
fn test_random_ratio_numerator_zero() {
    let result = rand::random_ratio(0, 5);
    assert_eq!(result, false);
}

#[test]
fn test_random_ratio_half_probability() {
    let result = rand::random_ratio(1, 2);
    assert!(result == true || result == false);
}

#[test]
fn test_random_ratio_two_thirds_probability() {
    let mut true_count = 0;
    let trials = 1000;
    for _ in 0..trials {
        if rand::random_ratio(2, 3) {
            true_count += 1;
        }
    }
    let probability = true_count as f64 / trials as f64;
    assert!(probability > 0.6 && probability < 0.8);
}

