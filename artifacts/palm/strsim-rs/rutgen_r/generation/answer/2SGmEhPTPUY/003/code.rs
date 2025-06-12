// Answer 0

#[test]
fn test_empty_sequences() {
    assert_eq!(generic_levenshtein(&[], &[]), 0);
}

#[test]
fn test_empty_first_sequence() {
    assert_eq!(generic_levenshtein(&[], &[1, 2, 3]), 3);
}

#[test]
fn test_empty_second_sequence() {
    assert_eq!(generic_levenshtein(&[1, 2, 3], &[]), 3);
}

#[test]
fn test_identical_sequences() {
    assert_eq!(generic_levenshtein(&[1, 2, 3], &[1, 2, 3]), 0);
}

#[test]
fn test_single_insertion() {
    assert_eq!(generic_levenshtein(&[1, 2, 3], &[1, 2, 3, 4]), 1);
}

#[test]
fn test_single_deletion() {
    assert_eq!(generic_levenshtein(&[1, 2, 3, 4], &[1, 2, 3]), 1);
}

#[test]
fn test_single_substitution() {
    assert_eq!(generic_levenshtein(&[1, 2, 3], &[1, 2, 4]), 1);
}

#[test]
fn test_multiple_operations() {
    assert_eq!(generic_levenshtein(&[1, 3], &[2, 4]), 2);
}

#[test]
fn test_large_sequences_with_differences() {
    let seq1: Vec<i32> = (1..1000).collect();
    let seq2: Vec<i32> = (500..1500).collect();
    assert_eq!(generic_levenshtein(&seq1, &seq2), 1000);
}

#[test]
#[should_panic]
fn test_panic_on_invalid_types() {
    let seq1 = &[1, 2, 3];
    let seq2 = &["a", "b", "c"];
    generic_levenshtein(seq1, seq2);
}

