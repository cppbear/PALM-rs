// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("ab", "bca"), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    assert_eq!(osa_distance("", ""), 0);
    assert_eq!(osa_distance("abc", ""), 3);
    assert_eq!(osa_distance("", "abc"), 3);
}

#[test]
fn test_osa_distance_identical_strings() {
    assert_eq!(osa_distance("test", "test"), 0);
}

#[test]
fn test_osa_distance_different_lengths() {
    assert_eq!(osa_distance("abc", "ab"), 1);
    assert_eq!(osa_distance("abcd", "abc"), 1);
    assert_eq!(osa_distance("a", "abc"), 2);
}

#[test]
fn test_osa_distance_with_transposition() {
    assert_eq!(osa_distance("abc", "bac"), 1);
    assert_eq!(osa_distance("ab", "ba"), 1);
    assert_eq!(osa_distance("abcde", "badce"), 3);
}

#[test]
fn test_osa_distance_special_characters() {
    assert_eq!(osa_distance("abc@", "bca!"), 4);
    assert_eq!(osa_distance("!@#", "#@!"), 3);
}

#[test]
fn test_osa_distance_max_runtime_conditions() {
    assert_eq!(osa_distance("abcdefghij", "jihgfedcba"), 10); // All different
    assert_eq!(osa_distance("test", "tset"), 1); // Permutation causing one swap
}

