// Answer 0

#[test]
fn test_osa_distance_example_case() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
}

#[test]
fn test_osa_distance_empty_first_string() {
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_empty_second_string() {
    assert_eq!(osa_distance("abc", ""), 3);
}

#[test]
fn test_osa_distance_single_character_different() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_single_character_same() {
    assert_eq!(osa_distance("a", "a"), 0);
}

#[test]
fn test_osa_distance_two_characters_transposed() {
    assert_eq!(osa_distance("ab", "ba"), 1);
}

#[test]
fn test_osa_distance_two_characters_different() {
    assert_eq!(osa_distance("ab", "cd"), 2);
}

#[test]
fn test_osa_distance_longer_strings() {
    assert_eq!(osa_distance("kitten", "sitting"), 3);
}

#[test]
fn test_osa_distance_with_adjacent_transpositions() {
    assert_eq!(osa_distance("abcdefg", "abdcfeg"), 2);
}

#[test]
fn test_osa_distance_multiple_adjacent_transpositions() {
    assert_eq!(osa_distance("abcde", "acdbe"), 3);
}

