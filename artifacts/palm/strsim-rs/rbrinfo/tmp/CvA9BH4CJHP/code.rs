pub fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}