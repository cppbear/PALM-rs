// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let s1: Vec<char> = "".chars().collect();
    let s2: Vec<char> = "".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 0, s2.iter().cloned(), 0), 0);
}

#[test]
fn test_damerau_levenshtein_one_empty_string() {
    let s1: Vec<char> = "abc".chars().collect();
    let s2: Vec<char> = "".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 3, s2.iter().cloned(), 0), 3);

    let s1: Vec<char> = "".chars().collect();
    let s2: Vec<char> = "abc".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 0, s2.iter().cloned(), 3), 3);
}

#[test]
fn test_damerau_levenshtein_identical_strings() {
    let s1: Vec<char> = "test".chars().collect();
    let s2: Vec<char> = "test".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 4, s2.iter().cloned(), 4), 0);
}

#[test]
fn test_damerau_levenshtein_single_character_insert() {
    let s1: Vec<char> = "a".chars().collect();
    let s2: Vec<char> = "ab".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 1, s2.iter().cloned(), 2), 1);
}

#[test]
fn test_damerau_levenshtein_single_character_delete() {
    let s1: Vec<char> = "ab".chars().collect();
    let s2: Vec<char> = "a".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 2, s2.iter().cloned(), 1), 1);
}

#[test]
fn test_damerau_levenshtein_single_character_substitute() {
    let s1: Vec<char> = "a".chars().collect();
    let s2: Vec<char> = "b".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 1, s2.iter().cloned(), 1), 1);
}

#[test]
fn test_damerau_levenshtein_multiple_operations() {
    let s1: Vec<char> = "kitten".chars().collect();
    let s2: Vec<char> = "sitting".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 6, s2.iter().cloned(), 7), 3);
}

#[test]
fn test_damerau_levenshtein_covariate_transpositions() {
    let s1: Vec<char> = "ca".chars().collect();
    let s2: Vec<char> = "ac".chars().collect();
    assert_eq!(damerau_levenshtein_impl(s1.iter().cloned(), 2, s2.iter().cloned(), 2), 1);
}

