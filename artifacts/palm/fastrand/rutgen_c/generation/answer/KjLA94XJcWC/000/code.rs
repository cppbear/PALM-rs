// Answer 0

#[test]
fn test_choose_multiple_from_std_vector() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3, 4, 5];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
    for item in result.iter() {
        assert!(source.contains(item));
    }
}

#[test]
fn test_choose_multiple_less_than_amount() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), source.len());
    assert_eq!(result[0], 1);
}

#[test]
fn test_choose_multiple_exact_amount() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
    assert!(result.iter().all(|item| source.contains(item)));
}

#[test]
fn test_choose_multiple_no_elements() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<usize> = vec![];
    let amount = 0;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), amount);
}

