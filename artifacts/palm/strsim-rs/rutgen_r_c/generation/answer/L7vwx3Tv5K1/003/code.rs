// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_one_empty_string() {
    assert_eq!(osa_distance("a", ""), 1);
    assert_eq!(osa_distance("", "a"), 1);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_different_strings() {
    assert_eq!(osa_distance("abc", "def"), 3);
}

#[test]
fn test_osa_distance_with_transposition() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_longer_strings() {
    assert_eq!(osa_distance("kitten", "sitting"), 3);
    assert_eq!(osa_distance("flaw", "lawn"), 2);
}

#[test]
fn test_osa_distance_strings_with_repeated_chars() {
    assert_eq!(osa_distance("aaa", "aaaa"), 1);
    assert_eq!(osa_distance("aab", "abb"), 1);
}

