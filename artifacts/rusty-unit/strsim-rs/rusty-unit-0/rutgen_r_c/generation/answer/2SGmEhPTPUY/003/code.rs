// Answer 0

#[test]
fn test_generic_levenshtein_identical_sequences() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_insertions() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_deletions() {
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_substitutions() {
    let a = vec![1, 2, 3];
    let b = vec![1, 3, 2];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_levenshtein_empty_first_sequence() {
    let a: Vec<i32> = vec![];
    let b = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_empty_second_sequence() {
    let a = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_empty_sequences() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_large_differences() {
    let a = vec![1];
    let b = vec![2, 3, 4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 5);
}

#[test]
fn test_generic_levenshtein_large_sequences() {
    let a: Vec<i32> = (1..=1000).collect();
    let b: Vec<i32> = (1001..=2000).collect();
    assert_eq!(generic_levenshtein(&a, &b), 1000);
}

