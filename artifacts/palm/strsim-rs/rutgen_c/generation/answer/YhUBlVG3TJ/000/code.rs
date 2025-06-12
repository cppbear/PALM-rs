// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(damerau_levenshtein("", ""), 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    assert_eq!(damerau_levenshtein("abc", ""), 3);
    assert_eq!(damerau_levenshtein("", "abc"), 3);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    assert_eq!(damerau_levenshtein("test", "test"), 0);
}

#[test]
fn test_damerau_levenshtein_single_character_change() {
    assert_eq!(damerau_levenshtein("a", "b"), 1);
}

#[test]
fn test_damerau_levenshtein_multiple_characters() {
    assert_eq!(damerau_levenshtein("kitten", "sitting"), 3);
    assert_eq!(damerau_levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_damerau_levenshtein_substring_edit() {
    assert_eq!(damerau_levenshtein("abcdef", "azced"), 3);
}

#[test]
fn test_damerau_levenshtein_long_strings() {
    assert_eq!(damerau_levenshtein("abcdefghijk", "abcdefghijk"), 0);
    assert_eq!(damerau_levenshtein("abcdefghij", "abcdefghijklm"), 3);
}

