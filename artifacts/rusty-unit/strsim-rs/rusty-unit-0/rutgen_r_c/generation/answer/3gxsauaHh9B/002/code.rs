// Answer 0

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b() {
    let a = "";
    let b = "nonempty";
    assert!((normalized_levenshtein(a, b) - 0.88889).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_a_empty_b() {
    let a = "";
    let b = "";
    assert!((normalized_levenshtein(a, b) - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_non_empty_a_empty_b() {
    let a = "nonempty";
    let b = "";
    assert!((normalized_levenshtein(a, b) - 0.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_same_strings() {
    let a = "string";
    let b = "string";
    assert!((normalized_levenshtein(a, b) - 1.0).abs() < 0.00001);
}

