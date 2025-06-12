// Answer 0

#[test]
fn test_f64_generated_value() {
    let value = fastrand::f64();
    assert!(value >= 0.0 && value < 1.0);
}

#[test]
fn test_f64_multiple_values() {
    let values: Vec<f64> = (0..100).map(|_| fastrand::f64()).collect();
    for value in values {
        assert!(value >= 0.0 && value < 1.0);
    }
}

#[test]
#[should_panic]
fn test_f64_edge_case() {
    // In this test, we expect no panic, but if the implementation 
    // somehow leads to generating exactly 1.0 or a negative value, 
    // it would cause this assertion to fail.
    let value = fastrand::f64();
    assert!(value >= 0.0 && value < 1.0);
}

