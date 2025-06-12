// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_one_empty_string() {
    assert_eq!(osa_distance("abc", ""), 3);
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_same_strings() {
    assert_eq!(osa_distance("abcd", "abcd"), 0);
}

#[test]
fn test_osa_distance_different_strings() {
    assert_eq!(osa_distance("abc", "bca"), 2);
    assert_eq!(osa_distance("abc", "def"), 3);
    assert_eq!(osa_distance("kitten", "sitting"), 3);
}

#[test]
fn test_osa_distance_adjacent_transpositions() {
    assert_eq!(osa_distance("ab", "ba"), 1);
    assert_eq!(osa_distance("abc", "acb"), 1);
}

#[test]
fn test_osa_distance_with_repeated_characters() {
    assert_eq!(osa_distance("aaab", "ab"), 2);
    assert_eq!(osa_distance("ab", "aaab"), 3);
}

#[test]
fn test_osa_distance_longer_strings() {
    assert_eq!(osa_distance("abcdefgh", "hgfedcba"), 8);
    assert_eq!(osa_distance("rust", "trust"), 2);
}

#[test]
#[should_panic]
fn test_osa_distance_panics_out_of_bounds() {
    let a: &str = "abc";
    let b: &str = "defghijklmnopqrstuvwxyz";
    osa_distance(a, b);
}

#[test]
#[should_panic]
fn test_osa_distance_panics_large_input() {
    let a: &str = "a".repeat(10000);
    let b: &str = "b".repeat(10000);
    osa_distance(a, b);
}

