// Answer 0

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("test", "test"), 0);
}

#[test]
fn test_osa_distance_single_character_difference() {
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_insert_character() {
    assert_eq!(osa_distance("abc", "ab"), 1);
}

#[test]
fn test_osa_distance_delete_character() {
    assert_eq!(osa_distance("ab", "abc"), 1);
}

#[test]
fn test_osa_distance_adjacent_transposition() {
    assert_eq!(osa_distance("abc", "bac"), 1);
}

#[test]
fn test_osa_distance_multiple_operations() {
    assert_eq!(osa_distance("kitten", "sitting"), 3);
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
fn test_osa_distance_large_difference() {
    assert_eq!(osa_distance("abcdefg", "hijklmnop"), 15);
}

