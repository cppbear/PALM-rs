// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    assert_eq!(damerau_levenshtein("", ""), 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    assert_eq!(damerau_levenshtein("test", ""), 4);
    assert_eq!(damerau_levenshtein("", "test"), 4);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    assert_eq!(damerau_levenshtein("example", "example"), 0);
}

#[test]
fn test_damerau_levenshtein_single_character_insert() {
    assert_eq!(damerau_levenshtein("a", "ab"), 1);
    assert_eq!(damerau_levenshtein("ab", "a"), 1);
}

#[test]
fn test_damerau_levenshtein_single_character_substitution() {
    assert_eq!(damerau_levenshtein("a", "b"), 1);
}

#[test]
fn test_damerau_levenshtein_single_character_deletion() {
    assert_eq!(damerau_levenshtein("ab", "b"), 1);
    assert_eq!(damerau_levenshtein("b", "ab"), 1);
}

#[test]
fn test_damerau_levenshtein_multiple_operations() {
    assert_eq!(damerau_levenshtein("kitten", "sitting"), 3);
    assert_eq!(damerau_levenshtein("flaw", "lawn"), 2);
}

#[test]
fn test_damerau_levenshtein_with_repeated_characters() {
    assert_eq!(damerau_levenshtein("aaaa", "aa"), 2);
    assert_eq!(damerau_levenshtein("aabb", "ab"), 2);
}

#[test]
fn test_damerau_levenshtein_case_sensitivity() {
    assert_eq!(damerau_levenshtein("test", "Test"), 1);
}

#[test]
fn test_damerau_levenshtein_large_input() {
    let a = "a".repeat(1000);
    let b = "a".repeat(500) + "b" + &"a".repeat(499);
    assert_eq!(damerau_levenshtein(&a, &b), 1);
}

