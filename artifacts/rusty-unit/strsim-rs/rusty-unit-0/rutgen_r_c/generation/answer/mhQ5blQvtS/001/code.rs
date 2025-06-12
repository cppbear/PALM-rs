// Answer 0

#[test]
fn test_damerau_levenshtein_same_strings() {
    let s1 = "test".chars();
    let s2 = "test".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_one_insertion() {
    let s1 = "test".chars();
    let s2 = "tests".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_one_deletion() {
    let s1 = "tests".chars();
    let s2 = "test".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_one_substitution() {
    let s1 = "test".chars();
    let s2 = "best".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "ab".chars();
    let s2 = "ba".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_multiple_edits() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_s1_empty() {
    let s1 = "".chars();
    let s2 = "test".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, len2); // expected: 4
}

#[test]
fn test_damerau_levenshtein_s2_empty() {
    let s1 = "test".chars();
    let s2 = "".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    let result = damerau_levenshtein_impl(s1, len1, s2, len2);
    assert_eq!(result, len1); // expected: 4
}

