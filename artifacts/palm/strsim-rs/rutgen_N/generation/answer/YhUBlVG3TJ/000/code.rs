// Answer 0

#[test]
fn test_damerau_levenshtein_same_strings() {
    assert_eq!(damerau_levenshtein("test", "test"), 0);
}

#[test]
fn test_damerau_levenshtein_one_insert() {
    assert_eq!(damerau_levenshtein("test", "atest"), 1);
}

#[test]
fn test_damerau_levenshtein_one_delete() {
    assert_eq!(damerau_levenshtein("atest", "test"), 1);
}

#[test]
fn test_damerau_levenshtein_one_replace() {
    assert_eq!(damerau_levenshtein("test", "tast"), 1);
}

#[test]
fn test_damerau_levenshtein_complex_case() {
    assert_eq!(damerau_levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(damerau_levenshtein("", ""), 0);
    assert_eq!(damerau_levenshtein("abc", ""), 3);
    assert_eq!(damerau_levenshtein("", "abc"), 3);
}

#[test]
fn test_damerau_levenshtein_different_lengths() {
    assert_eq!(damerau_levenshtein("short", "longer"), 3);
}

#[test]
fn test_damerau_levenshtein_edge_case() {
    assert_eq!(damerau_levenshtein("a", "a"), 0);
    assert_eq!(damerau_levenshtein("ab", "a"), 1);
    assert_eq!(damerau_levenshtein("a", "ab"), 1);
    assert_eq!(damerau_levenshtein("abc", "def"), 3);
}

