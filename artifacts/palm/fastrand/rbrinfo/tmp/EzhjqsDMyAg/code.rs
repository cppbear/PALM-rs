pub fn fill(slice: &mut [u8]) {
    with_rng(|r| r.fill(slice))
}