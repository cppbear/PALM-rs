// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1: String = "".to_string();
    let s2: String = "".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1: String = "abc".to_string();
    let s2: String = "".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1: String = "kitten".to_string();
    let s2: String = "kitten".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_single_char_insert() {
    let s1: String = "a".to_string();
    let s2: String = "ab".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_char_replace() {
    let s1: String = "a".to_string();
    let s2: String = "b".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_double_char_swap() {
    let s1: String = "ab".to_string();
    let s2: String = "ba".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 2);
}

#[test]
fn test_damerau_levenshtein_longer_strings() {
    let s1: String = "saturday".to_string();
    let s2: String = "sunday".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 3);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_panic_condition() {
    let s1: String = "abc".to_string();
    let s2: String = "abcd".to_string();
    let result = damerau_levenshtein_impl(s1.chars(), s1.len(), s2.chars(), s2.len());
    assert_eq!(result, 1); // This test is meant to check for panic, so no assertion needed
}

