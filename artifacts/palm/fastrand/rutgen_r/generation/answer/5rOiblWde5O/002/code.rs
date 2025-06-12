// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let mut rng = fastrand::Rng::new();
    let iter: Vec<i32> = Vec::new(); // An empty iterator
    let result = rng.choice(iter);
    assert_eq!(result, None);
}

