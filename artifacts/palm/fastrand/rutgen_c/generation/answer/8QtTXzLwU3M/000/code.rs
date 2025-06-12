// Answer 0

#[test]
fn test_fill_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 0] = [];
    rng.fill(&mut slice);
    assert!(slice.is_empty());
}

#[test]
fn test_fill_slice_of_size_1() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 1] = [0];
    rng.fill(&mut slice);
    assert!(slice != [0]);
}

#[test]
fn test_fill_slice_of_size_7() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 7] = [0; 7];
    rng.fill(&mut slice);
    assert!(slice != [0; 7]);
}

#[test]
fn test_fill_slice_of_size_8() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 8] = [0; 8];
    rng.fill(&mut slice);
    assert!(slice != [0; 8]);
}

#[test]
fn test_fill_slice_of_size_9() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 9] = [0; 9];
    rng.fill(&mut slice);
    assert!(slice != [0; 9]);
}

#[test]
fn test_fill_slice_of_size_15() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 15] = [0; 15];
    rng.fill(&mut slice);
    assert!(slice != [0; 15]);
}

#[test]
fn test_fill_slice_of_size_16() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 16] = [0; 16];
    rng.fill(&mut slice);
    assert!(slice != [0; 16]);
}

