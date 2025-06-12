// Answer 0

#[test]
fn test_choose_multiple_with_sufficient_elements() {
    let mut rng = fastrand::Rng::default();
    let source = vec![1, 2, 3, 4, 5];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
    for &item in &result {
        assert!(source.contains(&item));
    }
}

#[test]
fn test_choose_multiple_with_exact_elements() {
    let mut rng = fastrand::Rng::default();
    let source = vec![1, 2, 3];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
    for &item in &result {
        assert!(source.contains(&item));
    }
}

#[test]
fn test_choose_multiple_with_insufficient_elements() {
    let mut rng = fastrand::Rng::default();
    let source = vec![1, 2];
    let amount = 5;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), source.len());
    for &item in &result {
        assert!(source.contains(&item));
    }
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let mut rng = fastrand::Rng::default();
    let source = vec![1, 2, 3];
    let amount = 0;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
}

#[test]
fn test_choose_multiple_from_empty_source() {
    let mut rng = fastrand::Rng::default();
    let source: Vec<i32> = Vec::new();
    let amount = 5;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), source.len());
}

