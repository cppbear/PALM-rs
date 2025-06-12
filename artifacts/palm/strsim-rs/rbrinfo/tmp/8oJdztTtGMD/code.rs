pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    generic_jaro_winkler(&StringWrapper(a), &StringWrapper(b))
}