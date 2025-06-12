// Answer 0

#[test]
fn test_osa_distance_basic_case() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("", "abc"), 3);
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_single_character() {
    assert_eq!(osa_distance("a", "b"), 1);
    assert_eq!(osa_distance("a", "a"), 0);
}

#[test]
fn test_osa_distance_two_characters() {
    assert_eq!(osa_distance("ab", "ab"), 0);
    assert_eq!(osa_distance("ab", "ba"), 1);
    assert_eq!(osa_distance("ab", "cc"), 2);
}

#[test]
fn test_osa_distance_adjacent_transpositions() {
    assert_eq!(osa_distance("abc", "acb"), 1);
    assert_eq!(osa_distance("abc", "cab"), 2);
}

#[test]
fn test_osa_distance_large_difference() {
    assert_eq!(osa_distance("abcd", "xyz"), 6);
    assert_eq!(osa_distance("abcdef", "ghijklm"), 12);
}

#[test]
fn test_osa_distance_same_length_different_chars() {
    assert_eq!(osa_distance("abcd", "efgh"), 8);
}

#[test]
fn test_osa_distance_with_repeated_chars() {
    assert_eq!(osa_distance("aabbcc", "abcabc"), 3);
}

#[test]
fn test_osa_distance_edge_cases() {
    assert_eq!(osa_distance("a" , ""), 1);
    assert_eq!(osa_distance("abc" , "abc"), 0);
    assert_eq!(osa_distance("abc" , "xyz"), 6);
    assert_eq!(osa_distance("" , "a"), 1);
}

#[test]
#[should_panic]
fn test_osa_distance_panic_empty_second_string() {
    let _ = osa_distance("non-empty", "");
}

#[test]
#[should_panic]
fn test_osa_distance_panic_large_index() {
    // This test is for exploratory purposes to check panic conditions
    let _ = osa_distance("a", "b");
    let _ = osa_distance("ab", "bcdefghijklmnop");
}

