// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_both_empty() {
    normalized_damerau_levenshtein("", "");
}

#[test]
fn test_normalized_damerau_levenshtein_a_empty() {
    normalized_damerau_levenshtein("", "notempty");
}

#[test]
fn test_normalized_damerau_levenshtein_b_empty() {
    normalized_damerau_levenshtein("notempty", "");
}

