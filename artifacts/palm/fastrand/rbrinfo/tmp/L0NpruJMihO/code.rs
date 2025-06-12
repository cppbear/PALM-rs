pub fn alphanumeric() -> char {
    with_rng(|r| r.alphanumeric())
}