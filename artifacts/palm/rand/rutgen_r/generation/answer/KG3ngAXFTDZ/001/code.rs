// Answer 0

#[test]
fn test_random_bool_valid_true() {
    let p = 1.0; // Maximum probability, should always return true
    assert_eq!(random_bool(p), true);
}

#[test]
fn test_random_bool_valid_false() {
    let p = 0.0; // Minimum probability, should always return false
    assert_eq!(random_bool(p), false);
}

#[test]
fn test_random_bool_valid_boundary_low() {
    let p = 0.5; // Boundary case, expected to return true or false
    let result = random_bool(p);
    assert!(result == true || result == false);
}

#[test]
fn test_random_bool_valid_boundary_high() {
    let p = 0.999; // Just below the upper limit, expected to be mostly true
    let results = (0..1000).map(|_| random_bool(p)).collect::<Vec<_>>();
    let true_count = results.iter().filter(|&&b| b).count();
    assert!(true_count > 900); // Expecting a high count of true
}

#[should_panic]
fn test_random_bool_invalid_negative() {
    let p = -0.1; // Negative probability, should panic
    random_bool(p);
}

#[should_panic]
fn test_random_bool_invalid_above_one() {
    let p = 1.1; // Probability above 1, should panic
    random_bool(p);
}

