// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: Vec<u8> = vec![];
    rng.shuffle(&mut slice);
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: Vec<u8> = vec![1];
    rng.shuffle(&mut slice);
}

