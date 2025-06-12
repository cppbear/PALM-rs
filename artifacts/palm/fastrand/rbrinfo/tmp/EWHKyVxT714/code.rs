pub fn get_seed() -> u64 {
    with_rng(|r| r.get_seed())
}