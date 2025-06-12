// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("ab", "bca"), 3);
    assert_eq!(osa_distance("abc", "abc"), 0);
    assert_eq!(osa_distance("kitten", "sitting"), 3);
    assert_eq!(osa_distance("", "nonempty"), 8);
    assert_eq!(osa_distance("nonempty", ""), 8);
}

#[test]
fn test_osa_distance_edge_cases() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("a", "a"), 0);
    assert_eq!(osa_distance("a", "b"), 1);
    assert_eq!(osa_distance("abc", "ab"), 1);
    assert_eq!(osa_distance("ab", "abc"), 1);
}

#[test]
fn test_osa_distance_transpositions() {
    assert_eq!(osa_distance("abcdef", "badcfe"), 5);
    assert_eq!(osa_distance("abc", "cba"), 2);
    assert_eq!(osa_distance("aa", "ab"), 1);
    assert_eq!(osa_distance("ab", "aa"), 1);
}

#[test]
fn test_osa_distance_repeated_chars() {
    assert_eq!(osa_distance("aaab", "abab"), 2);
    assert_eq!(osa_distance("aaa", "aab"), 1);
    assert_eq!(osa_distance("a", "aaaa"), 3);
    assert_eq!(osa_distance("aaaa", "a"), 3);
}

