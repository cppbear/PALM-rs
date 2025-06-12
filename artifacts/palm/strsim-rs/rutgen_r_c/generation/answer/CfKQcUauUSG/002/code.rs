// Answer 0

#[test]
fn test_generic_damerau_levenshtein_empty_first_argument() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![1, 2, 3];

    let result = generic_damerau_levenshtein(&a, &b);
    assert_eq!(result, a.len());
}

#[test]
fn test_generic_damerau_levenshtein_empty_first_argument_longer_second() {
    let a: Vec<char> = vec![];
    let b: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let result = generic_damerau_levenshtein(&a, &b);
    assert_eq!(result, a.len());
}

#[test]
fn test_generic_damerau_levenshtein_empty_first_argument_empty_second() {
    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![];

    let result = generic_damerau_levenshtein(&a, &b);
    assert_eq!(result, a.len());
}

