// Answer 0

#[test]
fn test_levenshtein_identical_strings() {
    assert_eq!(levenshtein("test", "test"), 0);
}

#[test]
fn test_levenshtein_single_change() {
    assert_eq!(levenshtein("cat", "bat"), 1);
}

#[test]
fn test_levenshtein_insertion() {
    assert_eq!(levenshtein("bat", "bath"), 1);
}

#[test]
fn test_levenshtein_deletion() {
    assert_eq!(levenshtein("bath", "bat"), 1);
}

#[test]
fn test_levenshtein_multiple_changes() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein("", ""), 0);
}

#[test]
fn test_levenshtein_first_empty() {
    assert_eq!(levenshtein("", "abc"), 3);
}

#[test]
fn test_levenshtein_second_empty() {
    assert_eq!(levenshtein("abc", ""), 3);
}

#[test]
fn test_levenshtein_longer_different_strings() {
    assert_eq!(levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_levenshtein_case_sensitive() {
    assert_eq!(levenshtein("hello", "Hello"), 1);
}

