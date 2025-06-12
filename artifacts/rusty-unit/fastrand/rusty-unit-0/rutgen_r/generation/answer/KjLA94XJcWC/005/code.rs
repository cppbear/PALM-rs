// Answer 0

#[test]
fn test_choose_multiple_with_insufficient_elements() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), source.len());
}

#[test]
fn test_choose_multiple_with_exact_capacity_constraint() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3];
    let amount = 1;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
    assert!(result.capacity() == 3 * result.len());
}

