// Answer 0

#[test]
fn test_generic_levenshtein_empty_strings() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_one_empty_string() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 3);

    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_identical_sequences() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_complete_differences() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 6);
}

#[test]
fn test_generic_levenshtein_partial_differences() {
    let a: Vec<i32> = vec![1, 3, 2];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_levenshtein_with_repetitions() {
    let a: Vec<i32> = vec![1, 2, 2, 3];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 1);
}

