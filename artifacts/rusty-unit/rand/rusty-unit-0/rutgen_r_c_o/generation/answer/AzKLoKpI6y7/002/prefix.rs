// Answer 0

#[test]
fn test_partial_shuffle_non_panicking() {
    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = rand::thread_rng();
    let amount = 2;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_all_elements() {
    let mut slice = [1, 2, 3, 4];
    let mut rng = rand::thread_rng();
    let amount = 4;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_zero_amount() {
    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = rand::thread_rng();
    let amount = 0;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_large_slice() {
    let mut slice: Vec<_> = (0..100).collect();
    let mut rng = rand::thread_rng();
    let amount = 10;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_edge_case() {
    let mut slice = [1, 2, 3];
    let mut rng = rand::thread_rng();
    let amount = 1;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, amount);
}

