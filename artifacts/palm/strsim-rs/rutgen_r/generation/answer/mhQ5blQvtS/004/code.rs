// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1 = "".chars();
    let s2 = "".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 0, s2, 0), 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1 = "a".chars();
    let s2 = "".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 1, s2, 0), 1);

    let s1 = "".chars();
    let s2 = "a".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 0, s2, 1), 1);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1 = "test".chars();
    let s2 = "test".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 4, s2, 4), 0);
}

#[test]
fn test_damerau_levenshtein_single_character_difference() {
    let s1 = "a".chars();
    let s2 = "b".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 1, s2, 1), 1);
}

#[test]
fn test_damerau_levenshtein_insertions() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 3, s2, 4), 1);
}

#[test]
fn test_damerau_levenshtein_deletions() {
    let s1 = "abcd".chars();
    let s2 = "abc".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 4, s2, 3), 1);
}

#[test]
fn test_damerau_levenshtein_substitutions() {
    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 6, s2, 7), 3);
}

#[test]
fn test_damerau_levenshtein_complex_case() {
    let s1 = "flaw".chars();
    let s2 = "lawn".chars();
    assert_eq!(damerau_levenshtein_impl(s1, 4, s2, 4), 2);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_panic_condition1() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    let _ = damerau_levenshtein_impl(s1.take(1), 1, s2.take(2), 2);
}

#[test]
#[should_panic]
fn test_damerau_levenshtein_panic_condition2() {
    let s1 = "abc".chars();
    let s2 = "abcd".chars();
    let _ = damerau_levenshtein_impl(s1.clone(), 3, s2.clone(), 4);
}

