// Answer 0

#[test]
fn test_choose_multiple_with_exact_amount() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3, 4, 5];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
    assert!(result.iter().all(|&x| x >= 1 && x <= 5));
}

#[test]
fn test_choose_multiple_with_insufficient_elements() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), source.len());
    assert!(result.iter().all(|&x| x >= 1 && x <= 3));
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3, 4, 5];
    let amount = 0;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

#[test]
fn test_choose_multiple_with_one_element() {
    let mut rng = fastrand::Rng::new();
    let source = vec![42];
    let amount = 1;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
    assert_eq!(result[0], 42);
}

#[test]
#[should_panic]
fn test_choose_multiple_panic_on_empty_source_with_positive_amount() {
    let mut rng = fastrand::Rng::new();
    let source: Vec<i32> = vec![];
    let amount = 1;
    let _result = rng.choose_multiple(source, amount);
}

