pub fn hamming(a: &str, b: &str) -> HammingResult {
    generic_hamming(a.chars(), b.chars())
}