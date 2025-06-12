// Answer 0

#[test]
fn test_osa_distance_basic_cases() {
    assert_eq!(osa_distance("ab", "bca"), 3);
    assert_eq!(osa_distance("abc", "abc"), 0);
    assert_eq!(osa_distance("abc", "abd"), 1);
    assert_eq!(osa_distance("a", ""), 1);
    assert_eq!(osa_distance("", "a"), 1);
}

#[test]
fn test_osa_distance_edge_cases() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("123", "321"), 2);
    assert_eq!(osa_distance("hello", "hlo"), 2);
    assert_eq!(osa_distance("kitten", "sitting"), 3);
    assert_eq!(osa_distance("flaw", "lawn"), 2);
}

#[test]
#[should_panic]
fn test_osa_distance_panic_cases() {
    let _ = osa_distance("x", ""); // Edge case expecting panic
    let _ = osa_distance("", "y"); // Edge case expecting panic
}

#[test]
fn test_osa_distance_maximum_diff() {
    assert_eq!(osa_distance("abcdef", "ghijkl"), 12);
}

