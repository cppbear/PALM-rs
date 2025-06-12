// Answer 0

#[test]
fn test_step_rng_new_positive_values() {
    let initial = 10;
    let increment = 5;
    let rng = StepRng::new(initial, increment);
    assert_eq!(rng, StepRng { v: 10, a: 5 });
}

#[test]
fn test_step_rng_new_zero_increment() {
    let initial = 20;
    let increment = 0;
    let rng = StepRng::new(initial, increment);
    assert_eq!(rng, StepRng { v: 20, a: 0 });
}

#[test]
fn test_step_rng_new_large_values() {
    let initial = u64::MAX;
    let increment = 1;
    let rng = StepRng::new(initial, increment);
    assert_eq!(rng, StepRng { v: u64::MAX, a: 1 });
}

#[test]
fn test_step_rng_new_edge_case_zero_initial() {
    let initial = 0;
    let increment = 1;
    let rng = StepRng::new(initial, increment);
    assert_eq!(rng, StepRng { v: 0, a: 1 });
}

#[test]
fn test_step_rng_new_negative_initial() {
    // Since the function does not accept negative values, this will not be applicable as u64 cannot be negative.
    // Simply use the max value above to confirm behavior.
    let initial = u64::MAX - 1;
    let increment = 0;
    let rng = StepRng::new(initial, increment);
    assert_eq!(rng, StepRng { v: u64::MAX - 1, a: 0 });
}

