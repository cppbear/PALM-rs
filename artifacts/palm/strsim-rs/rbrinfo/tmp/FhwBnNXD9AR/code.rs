pub fn jaro(a: &str, b: &str) -> f64 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}