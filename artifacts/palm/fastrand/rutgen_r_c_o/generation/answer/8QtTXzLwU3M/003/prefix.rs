// Answer 0

#[test]
fn test_fill_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u8] = &mut [];
    rng.fill(slice);
}

#[test]
fn test_fill_exactly_eight_bytes() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u8] = &mut [0; 8];
    rng.fill(slice);
}

#[test]
fn test_fill_multiple_of_eight_bytes() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u8] = &mut [0; 16];
    rng.fill(slice);
}

#[test]
fn test_fill_multiple_of_eight_bytes_large() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u8] = &mut [0; 32];
    rng.fill(slice);
}

