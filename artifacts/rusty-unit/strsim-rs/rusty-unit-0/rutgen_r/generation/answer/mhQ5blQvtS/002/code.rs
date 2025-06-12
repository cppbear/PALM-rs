// Answer 0

#[test]
fn test_damerau_levenshtein_basic_case() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 7);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "hello".chars();
    let s2 = "hello".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1 = "".chars();
    let s2 = "abcd".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 4);
    assert_eq!(result, 4);
}

#[test]
fn test_damerau_levenshtein_both_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_single_character_different() {
    let s1 = "a".chars();
    let s2 = "b".chars();
    let result = damerau_levenshtein_impl(s1, 1, s2, 1);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_character_identical() {
    let s1 = "a".chars();
    let s2 = "a".chars();
    let result = damerau_levenshtein_impl(s1, 1, s2, 1);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_large_difference() {
    let s1 = "abcdefg".chars();
    let s2 = "xyz".chars();
    let result = damerau_levenshtein_impl(s1, 7, s2, 3);
    assert_eq!(result, 7);
}

#[test]
fn test_damerau_levenshtein_complex_case() {
    let s1 = "dancer".chars();
    let s2 = "danger".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 6);
    assert_eq!(result, 2);
}

