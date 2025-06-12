// Answer 0

#[test]
fn test_damerau_levenshtein_simple_case() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "ab".chars();
    let len2 = 2;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_single_insertion() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "abcd".chars();
    let len2 = 4;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_single_deletion() {
    let s1 = "abcd".chars();
    let len1 = 4;
    let s2 = "abc".chars();
    let len2 = 3;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_single_substitution() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "aac".chars();
    let len2 = 3;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "acb".chars();
    let len2 = 3;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_longer_strings() {
    let s1 = "kitten".chars();
    let len1 = 6;
    let s2 = "sitting".chars();
    let len2 = 7;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_same_strings() {
    let s1 = "hello".chars();
    let len1 = 5;
    let s2 = "hello".chars();
    let len2 = 5;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_empty_first_string() {
    let s1 = "".chars();
    let len1 = 0;
    let s2 = "abc".chars();
    let len2 = 3;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_empty_second_string() {
    let s1 = "abc".chars();
    let len1 = 3;
    let s2 = "".chars();
    let len2 = 0;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_identical_single_character() {
    let s1 = "a".chars();
    let len1 = 1;
    let s2 = "a".chars();
    let len2 = 1;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

#[test]
fn test_damerau_levenshtein_different_single_character() {
    let s1 = "a".chars();
    let len1 = 1;
    let s2 = "b".chars();
    let len2 = 1;
    damerau_levenshtein_impl(s1, len1, s2, len2);
}

