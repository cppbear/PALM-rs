// Answer 0

#[test]
fn test_damerau_levenshtein_empty_strings() {
    let a = "";
    let b = "";
    damerau_levenshtein(a, b);
}

#[test]
fn test_damerau_levenshtein_one_characters() {
    let a = "a";
    let b = "b";
    damerau_levenshtein(a, b);
}

#[test]
fn test_damerau_levenshtein_multiple_characters_equal_length() {
    let a = "abcdefghij";
    let b = "jihgfedcba";
    damerau_levenshtein(a, b);
}

#[test]
fn test_damerau_levenshtein_large_equal_length() {
    let a = "a".repeat(100);
    let b = "b".repeat(100);
    damerau_levenshtein(&a, &b);
}

#[test]
fn test_damerau_levenshtein_large_different_length() {
    let a = "a".repeat(1000);
    let b = "b".repeat(10000);
    damerau_levenshtein(&a, &b);
}

#[test]
fn test_damerau_levenshtein_large_same_length() {
    let a = "abcde".repeat(200);
    let b = "edcba".repeat(200);
    damerau_levenshtein(&a, &b);
}

