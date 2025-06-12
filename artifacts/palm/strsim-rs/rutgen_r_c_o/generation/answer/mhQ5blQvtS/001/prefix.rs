// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "test".chars();
    let s2 = "test".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_single_character_different() {
    let s1 = "a".chars();
    let s2 = "b".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_single_character_same() {
    let s1 = "x".chars();
    let s2 = "x".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_insert_character() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_delete_character() {
    let s1 = "abcd".chars();
    let s2 = "abc".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_replace_character() {
    let s1 = "abc".chars();
    let s2 = "abd".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_complex_case() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_long_strings() {
    let s1 = "a".repeat(1000).chars();
    let s2 = "b".repeat(1000).chars();
    let len1 = s1.clone().count();
    let len2 = s2.clone().count();
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

