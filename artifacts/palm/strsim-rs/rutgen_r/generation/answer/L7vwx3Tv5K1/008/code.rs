// Answer 0

#[test]
fn test_osa_distance_basic() {
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
    assert_eq!(osa_distance("a", "a"), 0);
    assert_eq!(osa_distance("a", "b"), 1);
}

#[test]
fn test_osa_distance_adjacent_transposition() {
    assert_eq!(osa_distance("abc", "acb"), 1);
    assert_eq!(osa_distance("ab", "ba"), 1);
}

#[test]
fn test_osa_distance_complex_case() {
    assert_eq!(osa_distance("kitten", "sitting"), 5);
    assert_eq!(osa_distance("flaw", "lawn"), 2);
}

#[test]
fn test_osa_distance_boundary_conditions() {
    assert_eq!(osa_distance("abcde", "bcdea"), 5); // All characters shifted
    assert_eq!(osa_distance("abcd", "abcde"), 1); // Add one character
}

#[should_panic]
fn test_osa_distance_panic_condition() {
    // Intentionally create a situation that may lead to out-of-bounds
    let result = osa_distance("abc", "a"); // Should not panic, but set up for demonstration
    assert_eq!(result, 1); // Just checking the invalid input for panic context
}

