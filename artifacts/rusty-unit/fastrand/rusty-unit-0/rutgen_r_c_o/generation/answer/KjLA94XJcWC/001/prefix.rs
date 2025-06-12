// Answer 0

#[test]
fn test_choose_multiple_basic() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3, 4, 5];
    let amount = 3;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_exceeding_amount() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_exact_amount() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3];
    let amount = 3;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![];
    let amount = 3;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_one_element_source() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1];
    let amount = 1;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_large_amount() {
    let mut rng = Rng::with_seed(42);
    let source = (1..=100).collect::<Vec<_>>();
    let amount = 100;
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_reallocates() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3];
    let amount = 2;
    let result = rng.choose_multiple(source, amount);
}

