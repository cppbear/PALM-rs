// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_one_char_difference() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("abc", ""), 3);
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_multiple_differences() {
    assert_eq!(osa_distance("abc", "def"), 3);
}

#[test]
fn test_osa_distance_adjacent_transpositions() {
    assert_eq!(osa_distance("ab", "ba"), 1);
}

#[test]
fn test_osa_distance_with_repeated_characters() {
    assert_eq!(osa_distance("aa", "aa"), 0);
    assert_eq!(osa_distance("aaa", "aab"), 1);
    assert_eq!(osa_distance("aaaa", "aaab"), 1);
}

#[test]
fn test_osa_distance_longer_strings() {
    assert_eq!(osa_distance("kitten", "sitting"), 3);
}

#[test]
fn test_osa_distance_edge_case_transpositions() {
    assert_eq!(osa_distance("aa", "ab"), 1);
    assert_eq!(osa_distance("ab", "aa"), 1);
}

#[test]
fn test_osa_distance_different_lengths() {
    assert_eq!(osa_distance("abc", "abcd"), 1);
    assert_eq!(osa_distance("abcd", "abc"), 1);
}

