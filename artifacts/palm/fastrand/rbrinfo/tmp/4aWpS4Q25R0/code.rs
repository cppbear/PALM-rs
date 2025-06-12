pub fn digit(base: u32) -> char {
    with_rng(|r| r.digit(base))
}