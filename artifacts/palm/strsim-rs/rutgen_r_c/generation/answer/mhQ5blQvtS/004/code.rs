// Answer 0

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "hello".chars();
    let s2 = "hello".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_single_character_insertion() {
    let s1 = "hello".chars();
    let s2 = "helloo".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 6);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_character_deletion() {
    let s1 = "helloo".chars();
    let s2 = "hello".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 5);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_single_character_substitution() {
    let s1 = "hello".chars();
    let s2 = "hallo".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "abc".chars();
    let s2 = "acb".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_different_length_strings() {
    let s1 = "short".chars();
    let s2 = "longer".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 6);
    assert_eq!(result, 2);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_empty_first_string() {
    let s1 = "".chars();
    let s2 = "nonempty".chars();
    let _ = damerau_levenshtein_impl(s1, 0, s2, 8);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_empty_second_string() {
    let s1 = "nonempty".chars();
    let s2 = "".chars();
    let _ = damerau_levenshtein_impl(s1, 8, s2, 0);
}

#[test]
fn test_damerau_levenshtein_with_special_characters() {
    let s1 = "hello!".chars();
    let s2 = "hello@".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 6);
    assert_eq!(result, 1);
}

