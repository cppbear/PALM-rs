// Answer 0

#[test]
fn test_fill_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 0] = [];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_1() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 1] = [0; 1];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_7() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 7] = [0; 7];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_8() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 8] = [0; 8];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_9() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 9] = [0; 9];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_16() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 16] = [0; 16];
    rng.fill(&mut slice);
}

#[test]
fn test_fill_slice_of_size_64() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 64] = [0; 64];
    rng.fill(&mut slice);
}

