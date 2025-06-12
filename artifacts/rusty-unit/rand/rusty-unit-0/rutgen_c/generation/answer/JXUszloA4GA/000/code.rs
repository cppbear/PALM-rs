// Answer 0

#[test]
fn test_next_u64_initial_value() {
    let mut rng = StepRng { v: 10, a: 5 };
    let result = rng.next_u64();
    assert_eq!(result, 10);
}

#[test]
fn test_next_u64_second_call() {
    let mut rng = StepRng { v: 10, a: 5 };
    let _ = rng.next_u64(); // First call
    let result = rng.next_u64(); // Second call
    assert_eq!(result, 15);
}

#[test]
fn test_next_u64_boundary_condition() {
    let mut rng = StepRng { v: u64::MAX - 5, a: 10 };
    let result = rng.next_u64();
    assert_eq!(result, u64::MAX - 5);
    let next_result = rng.next_u64();
    assert_eq!(next_result, u64::MAX);
}

#[test]
fn test_next_u64_after_multiple_calls() {
    let mut rng = StepRng { v: 0, a: 3 };
    let values: Vec<u64> = (0..5).map(|_| rng.next_u64()).collect();
    let expected_values = vec![0, 3, 6, 9, 12];
    assert_eq!(values, expected_values);
}

