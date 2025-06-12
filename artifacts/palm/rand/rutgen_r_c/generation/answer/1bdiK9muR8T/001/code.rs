// Answer 0

#[test]
fn test_next_u32_initial_state() {
    let mut rng = StepRng { v: 0, a: 1 };
    let result = rng.next_u32();
    assert_eq!(result, 0);
}

#[test]
fn test_next_u32_after_one_call() {
    let mut rng = StepRng { v: 1, a: 1 };
    rng.next_u32(); // First call updates `v`
    let result = rng.next_u32();
    assert_eq!(result, 2);
}

#[test]
fn test_next_u32_multiple_calls() {
    let mut rng = StepRng { v: 5, a: 3 };
    rng.next_u32(); // returns 5
    rng.next_u32(); // returns 8
    let result = rng.next_u32(); // returns 11
    assert_eq!(result, 11);
}

#[test]
fn test_next_u32_with_large_values() {
    let mut rng = StepRng { v: u64::MAX - 1, a: 2 };
    let result = rng.next_u32(); // should return u64::MAX - 1 as u32
    assert_eq!(result, u32::MAX);
}

#[test]
fn test_next_u32_zero_increment() {
    let mut rng = StepRng { v: 10, a: 0 };
    let result = rng.next_u32(); // should always return 10
    assert_eq!(result, 10);
    let result_again = rng.next_u32(); // still should return 10
    assert_eq!(result_again, 10);
}

