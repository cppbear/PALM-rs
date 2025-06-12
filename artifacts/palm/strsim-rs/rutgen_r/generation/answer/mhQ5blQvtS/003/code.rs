// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1 = "test".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 4, s2, 0);
    assert_eq!(result, 4);
    
    let s1 = "".chars();
    let s2 = "test".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 4);
    assert_eq!(result, 4);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "racecar".chars();
    let s2 = "racecar".chars();
    let result = damerau_levenshtein_impl(s1, 7, s2, 7);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_single_substitution() {
    let s1 = "kitten".chars();
    let s2 = "sitten".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 6);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_deletion() {
    let s1 = "flaw".chars();
    let s2 = "law".chars();
    let result = damerau_levenshtein_impl(s1, 4, s2, 3);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_insertion() {
    let s1 = "law".chars();
    let s2 = "flaw".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 4);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_multiple_operations() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 7);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "abcd".chars();
    let s2 = "badc".chars();
    let result = damerau_levenshtein_impl(s1, 4, s2, 4);
    assert_eq!(result, 4);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_panic_condition() {
    let s1 = "test".chars();
    let s2 = "te".chars();
    let _result = damerau_levenshtein_impl(s1, 4, s2, 2);
}

