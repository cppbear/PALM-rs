// Answer 0

#[test]
fn test_choose_multiple_zero_amount() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<u32> = (1..=50).collect();
    let result = rng.choose_multiple(source, 0);
}

#[test]
fn test_choose_multiple_full_reservoir() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<u32> = (1..=50).collect();
    let result = rng.choose_multiple(source.clone(), 10);
}

#[test]
fn test_choose_multiple_exhausted_iterator() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<u32> = (1..=5).collect();
    let result = rng.choose_multiple(source.clone(), 10);
}

#[test]
fn test_choose_multiple_large_amount() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<u32> = (1..=100).collect();
    let result = rng.choose_multiple(source.clone(), 20);
}

#[test]
fn test_choose_multiple_amount_equals_source_length() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<u32> = (1..=50).collect();
    let result = rng.choose_multiple(source.clone(), 50);
}

