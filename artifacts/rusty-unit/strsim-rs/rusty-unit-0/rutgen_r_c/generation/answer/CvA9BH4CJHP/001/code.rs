// Answer 0

#[test]
fn test_levenshtein_identical_strings() {
    assert_eq!(levenshtein("test", "test"), 0);
}

#[test]
fn test_levenshtein_one_insertion() {
    assert_eq!(levenshtein("test", "tests"), 1);
}

#[test]
fn test_levenshtein_one_deletion() {
    assert_eq!(levenshtein("tests", "test"), 1);
}

#[test]
fn test_levenshtein_one_substitution() {
    assert_eq!(levenshtein("test", "best"), 1);
}

#[test]
fn test_levenshtein_insertion_and_deletion() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_multiple_operations() {
    assert_eq!(levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_levenshtein_empty_string_a() {
    assert_eq!(levenshtein("", "nonempty"), 8);
}

#[test]
fn test_levenshtein_empty_string_b() {
    assert_eq!(levenshtein("nonempty", ""), 8);
}

#[test]
fn test_levenshtein_different_lengths() {
    assert_eq!(levenshtein("abc", "abcdef"), 3);
}

#[test]
fn test_levenshtein_edge_case_empty_to_empty() {
    assert_eq!(levenshtein("", ""), 0);
}

