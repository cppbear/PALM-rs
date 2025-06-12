// Answer 0

#[test]
fn test_damerau_levenshtein_impl_identical_strings() {
    let s1 = "hello".chars();
    let s2 = "hello".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(result, 0);
}

#[test]
fn test_damerau_levenshtein_impl_insertions() {
    let s1 = "hello".chars();
    let s2 = "helloworld".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 10);
    assert_eq!(result, 5);
}

#[test]
fn test_damerau_levenshtein_impl_deletions() {
    let s1 = "helloworld".chars();
    let s2 = "hello".chars();
    let result = damerau_levenshtein_impl(s1, 10, s2, 5);
    assert_eq!(result, 5);
}

#[test]
fn test_damerau_levenshtein_impl_replacements() {
    let s1 = "hello".chars();
    let s2 = "hallo".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 5);
    assert_eq!(result, 1);
}

#[test]
fn test_damerau_levenshtein_impl_transpositions() {
    let s1 = "abcd".chars();
    let s2 = "badc".chars();
    let result = damerau_levenshtein_impl(s1, 4, s2, 4);
    assert_eq!(result, 2); // "ab" -> "ba" and "cd" -> "dc"
}

#[test]
fn test_damerau_levenshtein_impl_empty_strings() {
    let s1 = "".chars();
    let s2 = "abc".chars();
    let result = damerau_levenshtein_impl(s1, 0, s2, 3);
    assert_eq!(result, 3);

    let s1 = "abc".chars();
    let s2 = "".chars();
    let result = damerau_levenshtein_impl(s1, 3, s2, 0);
    assert_eq!(result, 3);
}

#[test]
fn test_damerau_levenshtein_impl_partial_overlap() {
    let s1 = "abcde".chars();
    let s2 = "cdeabc".chars();
    let result = damerau_levenshtein_impl(s1, 5, s2, 6);
    assert_eq!(result, 3); // require 3 operations to match
}

