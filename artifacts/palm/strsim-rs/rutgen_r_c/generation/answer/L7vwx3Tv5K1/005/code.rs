// Answer 0

#[test]
fn test_osa_distance_basic_case() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_first_empty_string() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_second_empty_string() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_single_character_difference() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_adjacent_transposition() {
    assert_eq!(osa_distance("abc", "acb"), 1);
}

#[test]
fn test_osa_distance_multiple_insertions() {
    assert_eq!(osa_distance("a", "abc"), 2);
}

#[test]
fn test_osa_distance_multiple_deletions() {
    assert_eq!(osa_distance("abc", "a"), 2);
}

#[test]
fn test_osa_distance_transposition_and_insertion() {
    assert_eq!(osa_distance("abc", "bacd"), 2);
}

