// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_first_empty() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_second_empty() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_no_change() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_one_change() {
    assert_eq!(osa_distance("abc", "abx"), 1);
}

#[test]
fn test_osa_distance_two_changes() {
    assert_eq!(osa_distance("abc", "xyz"), 3);
}

#[test]
fn test_osa_distance_with_transposition() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_with_transposition_no_cost() {
    assert_eq!(osa_distance("abcd", "bcda"), 1);
}

#[test]
fn test_osa_distance_with_whitespace() {
    assert_eq!(osa_distance("a b c", "abc"), 2);
}

#[test]
fn test_osa_distance_with_special_characters() {
    assert_eq!(osa_distance("a@bc", "abc"), 1);
}

