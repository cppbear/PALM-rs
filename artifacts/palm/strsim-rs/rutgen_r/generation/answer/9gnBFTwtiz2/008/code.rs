// Answer 0

#[derive(Debug)]
struct StrSimError;

type HammingResult = Result<usize, StrSimError>;

#[test]
fn test_generic_hamming_equal_sequences() {
    let seq1 = vec![1, 2, 3];
    let seq2 = vec![1, 2, 3];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_partial_match() {
    let seq1 = vec![1, 2, 3];
    let seq2 = vec![1, 2, 4];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_generic_hamming_different_length() {
    let seq1 = vec![1, 2, 3];
    let seq2 = vec![1, 2];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Err(StrSimError));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let seq1: Vec<i32> = vec![];
    let seq2: Vec<i32> = vec![];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_one_empty_one_non_empty() {
    let seq1 = vec![1, 2, 3];
    let seq2: Vec<i32> = vec![];
    let result = generic_hamming(seq1, seq2);
    assert_eq!(result, Err(StrSimError));
}

