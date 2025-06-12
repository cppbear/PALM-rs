// Answer 0

#[test]
fn test_choose_multiple_exact_amount() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3, 4, 5];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

#[test]
fn test_choose_multiple_less_than_amount() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), source.len());
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut rng = fastrand::Rng::new();
    let source: Vec<i32> = Vec::new();
    let amount = 0;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

#[test]
fn test_choose_multiple_exceeding_non_panic_capacity() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), source.len());
}

#[test]
fn test_choose_multiple_one_item() {
    let mut rng = fastrand::Rng::new();
    let source = vec![42];
    let amount = 1;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

#[test]
#[should_panic]
fn test_choose_multiple_zero_amount() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2, 3];
    let amount = 0;
    let _result = rng.choose_multiple(source, amount);
}

