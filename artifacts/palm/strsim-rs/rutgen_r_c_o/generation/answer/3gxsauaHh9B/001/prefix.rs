// Answer 0

#[test]
fn test_normalized_levenshtein_empty_strings() {
    let a = "";
    let b = "";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_empty_and_non_empty_string() {
    let a = "";
    let b = "second";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_non_empty_and_empty_string() {
    let a = "first";
    let b = "";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_identical_strings() {
    let a = "string";
    let b = "string";
    normalized_levenshtein(a, b);
}

#[test]
fn test_normalized_levenshtein_different_strings() {
    let a = "kitten";
    let b = "sitting";
    normalized_levenshtein(a, b);
}

