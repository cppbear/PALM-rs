// Answer 0

#[test]
fn test_damerau_levenshtein_impl_empty_strings() {
    let s1 = "";
    let len1 = s1.len();
    let s2 = "";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_impl_one_empty_string() {
    let s1 = "abc";
    let len1 = s1.len();
    let s2 = "";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, len1);
    
    let s1 = "";
    let len1 = s1.len();
    let s2 = "abc";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, len2);
}

#[test]
fn test_damerau_levenshtein_impl_identical_strings() {
    let s1 = "test";
    let len1 = s1.len();
    let s2 = "test";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_impl_single_character_difference() {
    let s1 = "abc";
    let len1 = s1.len();
    let s2 = "abd";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_impl_two_character_difference() {
    let s1 = "abc";
    let len1 = s1.len();
    let s2 = "xyz";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_transposition() {
    let s1 = "abc";
    let len1 = s1.len();
    let s2 = "acb";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_impl_longer_strings() {
    let s1 = "kitten";
    let len1 = s1.len();
    let s2 = "sitting";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_uppercase_vs_lowercase() {
    let s1 = "ABC";
    let len1 = s1.len();
    let s2 = "abc";
    let len2 = s2.len();
    let result = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
    assert_eq!(result, 3);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_impl_r_panics() {
    let s1 = "a";
    let len1 = s1.len();
    let s2 = "abcd";
    let len2 = s2.len();
    let _ = damerau_levenshtein_impl(s1.chars(), len1, s2.chars(), len2);
}

