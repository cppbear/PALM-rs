// Answer 0

#[test]
fn test_step_rng_new() {
    let initial = 10;
    let increment = 5;
    let rng = StepRng::new(initial, increment);

    assert_eq!(rng.v, initial);
    assert_eq!(rng.a, increment);
}

#[test]
fn test_step_rng_new_with_zero_increment() {
    let initial = 42;
    let increment = 0;
    let rng = StepRng::new(initial, increment);

    assert_eq!(rng.v, initial);
    assert_eq!(rng.a, increment);
}

#[test]
fn test_step_rng_new_with_large_values() {
    let initial = u64::MAX;
    let increment = 1;
    let rng = StepRng::new(initial, increment);

    assert_eq!(rng.v, initial);
    assert_eq!(rng.a, increment);
}

