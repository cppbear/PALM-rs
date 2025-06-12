// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("a", ""), 1);
    assert_eq!(osa_distance("", "a"), 1);
}

#[test]
fn test_osa_distance_same_strings() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_with_transpositions() {
    assert_eq!(osa_distance("abc", "acb"), 1);
    assert_eq!(osa_distance("cab", "abc"), 1);
    assert_eq!(osa_distance("aab", "aba"), 1);
}

#[test]
fn test_osa_distance_different_length() {
    assert_eq!(osa_distance("abc", "ab"), 1);
    assert_eq!(osa_distance("abcd", "abc"), 1);
}

#[test]
fn test_osa_distance_maximum_length() {
    let a = "a".repeat(1000);
    let b = "a".repeat(999) + "b";
    assert_eq!(osa_distance(&a, &b), 1);
}

