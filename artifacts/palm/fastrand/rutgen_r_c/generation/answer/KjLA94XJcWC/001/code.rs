// Answer 0

#[test]
fn test_choose_multiple_full_reservoir() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3, 4, 5];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_choose_multiple_partial_reservoir() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = Vec::new();
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_choose_multiple_small_source() {
    let mut rng = Rng::with_seed(42);
    let source = vec![10, 20];
    let amount = 3;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![10, 20]);
} 

#[test]
#[should_panic(expected = "empty range")]
fn test_choose_multiple_panic_on_empty_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = Vec::new();
    let amount = 0;
    let _result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_large_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = (1..=100).collect();
    let amount = 10;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

