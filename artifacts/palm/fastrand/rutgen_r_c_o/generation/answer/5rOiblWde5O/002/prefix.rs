// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let mut rng = Rng::with_seed(42);
    let vec: Vec<i32> = Vec::new();
    let result = rng.choice(vec);
}

#[test]
fn test_choice_empty_range() {
    let mut rng = Rng::with_seed(43);
    let result = rng.choice(std::iter::empty::<i32>());
}

