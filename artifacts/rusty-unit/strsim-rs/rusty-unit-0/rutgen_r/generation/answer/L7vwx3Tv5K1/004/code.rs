// Answer 0

#[test]
fn test_osa_distance_simple_cases() {
    assert_eq!(osa_distance("ab", "bca"), 3);
    assert_eq!(osa_distance("abc", "abc"), 0);
    assert_eq!(osa_distance("abc", "ab"), 1);
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_with_transposition() {
    assert_eq!(osa_distance("abc", "acb"), 1);
    assert_eq!(osa_distance("ab", "ba"), 1);
    assert_eq!(osa_distance("abcd", "abdc"), 1);
}

#[test]
fn test_osa_distance_edge_cases() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("a", ""), 1);
    assert_eq!(osa_distance("", "a"), 1);
}

#[test]
fn test_osa_distance_complex_cases() {
    assert_eq!(osa_distance("kitten", "sittin"), 3);
    assert_eq!(osa_distance("flaw", "lawn"), 2);
}

