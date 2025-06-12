// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let mut rng = Rng::with_seed(42);
    let result: Option<u32> = rng.choice(Vec::<u32>::new());
    assert_eq!(result, None);
}

#[test]
fn test_choice_single_element() {
    let mut rng = Rng::with_seed(42);
    let result: Option<u32> = rng.choice(vec![10]);
    assert_eq!(result, Some(10));
}

#[test]
fn test_choice_multiple_elements() {
    let mut rng = Rng::with_seed(42);
    let result: Option<u32> = rng.choice(vec![1, 2, 3, 4, 5]);
    assert_eq!(result, Some(3)); // Based on the seed, expected item can be adjusted
}

#[test]
fn test_choice_large_iterator() {
    let mut rng = Rng::with_seed(42);
    let result: Option<u32> = rng.choice((1..100).collect::<Vec<u32>>());
    assert!(result.is_some());
    assert!(result.unwrap() >= 1 && result.unwrap() <= 99);
}

