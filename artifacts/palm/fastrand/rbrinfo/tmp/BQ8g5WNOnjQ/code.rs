pub fn seed(seed: u64) {
    with_rng(|r| r.seed(seed));
}