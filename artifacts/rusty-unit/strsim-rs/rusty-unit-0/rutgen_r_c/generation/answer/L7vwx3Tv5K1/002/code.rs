// Answer 0

#[test]
fn test_osa_distance_basic_case() {
    let result = osa_distance("ab", "bca");
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_identical_strings() {
    let result = osa_distance("hello", "hello");
    assert_eq!(result, 0);
}

#[test]
fn test_osa_distance_one_empty_string() {
    let result = osa_distance("abc", "");
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_both_empty_strings() {
    let result = osa_distance("", "");
    assert_eq!(result, 0);
}

#[test]
fn test_osa_distance_single_character_transposition() {
    let result = osa_distance("ab", "ba");
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_longer_transposition() {
    let result = osa_distance("abcde", "bacde");
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_multiple_operations() {
    let result = osa_distance("kitten", "sitting");
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_single_character_difference() {
    let result = osa_distance("a", "b");
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_consecutive_operations() {
    let result = osa_distance("ab", "ac");
    assert_eq!(result, 2);
}

#[test]
fn test_osa_distance_boundary_case() {
    let result = osa_distance("abc", "def");
    assert_eq!(result, 6);
}

