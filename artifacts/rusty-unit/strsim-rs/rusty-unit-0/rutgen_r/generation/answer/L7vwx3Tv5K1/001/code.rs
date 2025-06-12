// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_single_character() {
    assert_eq!(osa_distance("a", "b"), 1);
    assert_eq!(osa_distance("b", "a"), 1);
}

#[test]
fn test_osa_distance_no_change() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_adjacent_transposition() {
    assert_eq!(osa_distance("ab", "ba"), 1); // adjacent swap
    assert_eq!(osa_distance("abc", "acb"), 1); // adjacent swap
}

#[test]
fn test_osa_distance_multiple_transpositions() {
    assert_eq!(osa_distance("abcde", "bacde"), 1); // single transposition
    assert_eq!(osa_distance("abcdef", "badcfe"), 3); // multiple transpositions
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("a", ""), 1);
    assert_eq!(osa_distance("", "b"), 1);
}

#[test]
fn test_osa_distance_complex_cases() {
    assert_eq!(osa_distance("kitten", "sitting"), 5);
    assert_eq!(osa_distance("flaw", "lawn"), 2);
}

