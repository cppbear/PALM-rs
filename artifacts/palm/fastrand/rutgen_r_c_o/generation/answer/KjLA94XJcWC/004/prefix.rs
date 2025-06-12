// Answer 0

#[test]
fn test_choose_multiple_insufficient_elements() {
    let mut rng = Rng::with_seed(123);
    let source = vec![1, 2]; // Only 2 elements available
    let amount = 3; // Requesting 3 elements
    let _result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_excess_capacity() {
    let mut rng = Rng::with_seed(456);
    let source = vec![1, 2, 3, 4, 5]; // 5 elements available
    let amount = 2; // Requesting 2 elements
    let _result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_reservoir_shrink() {
    let mut rng = Rng::with_seed(789);
    let source = (1..=10).collect::<Vec<_>>(); // 10 elements available
    let amount = 3; // Requesting 3 elements
    let _result = rng.choose_multiple(source, amount);
}

