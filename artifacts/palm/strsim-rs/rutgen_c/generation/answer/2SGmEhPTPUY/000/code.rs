// Answer 0

#[test]
fn test_generic_levenshtein_basic_differences() {
    assert_eq!(3, generic_levenshtein(&[1, 2, 3], &[1, 2, 3, 4, 5, 6]));
    assert_eq!(4, generic_levenshtein(&[1, 2, 3], &[4, 5, 6]));
    assert_eq!(0, generic_levenshtein(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn test_generic_levenshtein_empty_sequences() {
    assert_eq!(0, generic_levenshtein(&[], &[]));
    assert_eq!(3, generic_levenshtein(&[], &[1, 2, 3]));
    assert_eq!(3, generic_levenshtein(&[1, 2, 3], &[]));
}

#[test]
fn test_generic_levenshtein_identical_sequences() {
    assert_eq!(0, generic_levenshtein(&['a', 'b', 'c'], &['a', 'b', 'c']));
}

#[test]
fn test_generic_levenshtein_single_element_changes() {
    assert_eq!(1, generic_levenshtein(&[1], &[2]));
    assert_eq!(1, generic_levenshtein(&[1], &[1, 2]));
    assert_eq!(1, generic_levenshtein(&[1, 2], &[2]));
}

#[test]
fn test_generic_levenshtein_multiple_types() {
    assert_eq!(2, generic_levenshtein(&['a', 'b', 'c'], &['a', 'c', 'd']));
}

