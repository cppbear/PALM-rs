// Answer 0

#[test]
fn test_empty_strings() {
    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_a_empty() {
    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![1, 2, 3];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_b_empty() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = vec![];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_identical_sequences() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = vec![1, 2, 3];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_one_insert() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = vec![1, 2, 3, 4];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_one_delete() {
    let a: Vec<u8> = vec![1, 2, 3, 4];
    let b: Vec<u8> = vec![1, 2, 3];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_one_substitution() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = vec![1, 0, 3];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_multiple_changes() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = vec![0, 0, 0];
    generic_levenshtein(&a, &b);
}

#[test]
fn test_large_sequences() {
    let a: Vec<u8> = (1..=1000).collect();
    let b: Vec<u8> = (2..=1001).collect();
    generic_levenshtein(&a, &b);
}

