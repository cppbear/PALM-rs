// Answer 0

#[test]
fn test_choose_multiple_insufficient_elements() {
    let mut rng = Rng::with_seed(1);
    let source = vec![1, 2]; // source has fewer elements than specified amount
    let amount = 5; // request more elements than provided
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_exact_capacity() {
    let mut rng = Rng::with_seed(2);
    let source = (1..=6).collect::<Vec<_>>(); // source has exactly 3 * amount
    let amount = 2; // selecting 2 elements
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_capacity_boundary() {
    let mut rng = Rng::with_seed(3);
    let source = (1..=9).collect::<Vec<_>>(); // source has 3 * amount
    let amount = 3; // selecting 3 elements
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_reallocation() {
    let mut rng = Rng::with_seed(4);
    let source = (1..=8).collect::<Vec<_>>(); // source has less than 3 * amount
    let amount = 4; // selecting 4 elements
    let result = rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_random_selection() {
    let mut rng = Rng::with_seed(5);
    let source = (1..=15).collect::<Vec<_>>(); // source has more than 3 * amount
    let amount = 5; // selecting 5 elements
    let result = rng.choose_multiple(source, amount);
}

