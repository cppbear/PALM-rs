// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 0);
}

#[test]
fn test_damerau_levenshtein_first_string_empty() {
    let s1 = "".chars();
    let s2 = "abc".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 3);
}

#[test]
fn test_damerau_levenshtein_second_string_empty() {
    let s1 = "abc".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 0);
}

#[test]
fn test_damerau_levenshtein_different_characters() {
    let s1 = "abc".chars();
    let s2 = "def".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
}

#[test]
fn test_damerau_levenshtein_one_insert() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 4);
}

#[test]
fn test_damerau_levenshtein_one_delete() {
    let s1 = "abcd".chars();
    let s2 = "abc".chars();
    let result = damerau_levenshtein_impl(s1, 4, s2, 3);
}

#[test]
fn test_damerau_levenshtein_one_substitution() {
    let s1 = "abc".chars();
    let s2 = "axy".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
}

#[test]
fn test_damerau_levenshtein_two_substitutions() {
    let s1 = "abc".chars();
    let s2 = "xyz".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
}

#[test]
fn test_damerau_levenshtein_transposition() {
    let s1 = "abc".chars();
    let s2 = "acb".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 3);
}

#[test]
fn test_damerau_levenshtein_max_length_strings() {
    let s1 = "a".repeat(20).chars();
    let s2 = "b".repeat(20).chars();
    let result = damerau_levenshtein_impl(s1, 20, s2, 20);
}

#[test]
fn test_damerau_levenshtein_edge_case_max_length_diff_characters() {
    let s1 = "a".repeat(20).chars();
    let s2 = "b".repeat(20).chars();
    let result = damerau_levenshtein_impl(s1, 20, s2, 20);
}

