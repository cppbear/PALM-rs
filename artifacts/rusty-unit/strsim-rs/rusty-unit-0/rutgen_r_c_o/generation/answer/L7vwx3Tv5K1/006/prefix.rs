// Answer 0

#[test]
fn test_osa_distance_empty_strings() {
    osa_distance("", "");
}

#[test]
fn test_osa_distance_first_string_empty() {
    osa_distance("", "b");
}

#[test]
fn test_osa_distance_second_string_empty() {
    osa_distance("a", "");
}

#[test]
fn test_osa_distance_single_character_different() {
    osa_distance("a", "b");
}

#[test]
fn test_osa_distance_single_character_same() {
    osa_distance("a", "a");
}

#[test]
fn test_osa_distance_different_length_strings() {
    osa_distance("abc", "ab");
    osa_distance("ab", "abc");
}

#[test]
fn test_osa_distance_multiple_same_characters() {
    osa_distance("aaa", "aaa");
}

#[test]
fn test_osa_distance_different_chars_same_length() {
    osa_distance("ab", "ba");
}

#[test]
fn test_osa_distance_reverse_string() {
    osa_distance("abc", "cba");
}

#[test]
fn test_osa_distance_longer_reverse_string() {
    osa_distance("abcd", "dcba");
}

#[test]
fn test_osa_distance_equal_long_strings() {
    osa_distance("abcd", "abcd");
}

#[test]
fn test_osa_distance_extra_character_in_second_string() {
    osa_distance("aaa", "aaaa");
}

#[test]
fn test_osa_distance_extra_character_first() {
    osa_distance("aaa", "");
}

#[test]
fn test_osa_distance_extra_character_second() {
    osa_distance("", "aaa");
}

