// Answer 0

#[test]
fn test_fill_with_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 0] = [];
    rng.fill(&mut slice);
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_fill_with_exact_u64_chunk() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 8] = [0; 8];
    rng.fill(&mut slice);
    assert_eq!(slice.len(), 8);
}

#[test]
fn test_fill_with_slice_smaller_than_u64_chunk() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 7] = [0; 7];
    rng.fill(&mut slice);
    assert_eq!(slice.len(), 7);
}

#[test]
fn test_fill_with_slice_larger_than_u64_chunk() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 15] = [0; 15];
    rng.fill(&mut slice);
    assert_eq!(slice.len(), 15);
}

#[test]
fn test_fill_with_multiple_u64_chunks() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [u8; 16] = [0; 16];
    rng.fill(&mut slice);
    assert_eq!(slice.len(), 16);
}

