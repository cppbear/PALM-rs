// Answer 0

#[test]
fn test_random_bool_with_probability_zero() {
    let result = rand::random_bool(0.0);
    assert_eq!(result, false);
}

#[test]
fn test_random_bool_with_probability_one() {
    let result = rand::random_bool(1.0);
    assert_eq!(result, true);
}

#[should_panic(expected = "p=0.5 is outside range [0.0, 1.0]")]
#[test]
fn test_random_bool_with_negative_probability() {
    rand::random_bool(-0.1);
}

#[should_panic(expected = "p=1.5 is outside range [0.0, 1.0]")]
#[test]
fn test_random_bool_with_exceeding_probability() {
    rand::random_bool(1.5);
}

#[test]
fn test_random_bool_with_probability_half() {
    let p = 0.5;
    let trials = 1000;
    let mut true_count = 0;
    
    for _ in 0..trials {
        if rand::random_bool(p) {
            true_count += 1;
        }
    }
    
    let true_ratio = true_count as f64 / trials as f64;
    assert!((true_ratio - p).abs() < 0.1);
}

