// Answer 0

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b() {
    let a = "";
    let b = "second";
    assert!(normalized_levenshtein(a, b).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_a_empty_b() {
    let a = "";
    let b = "";
    assert!((normalized_levenshtein(a, b) - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_non_empty_a_empty_b() {
    let a = "first";
    let b = "";
    assert!(normalized_levenshtein(a, b).abs() < 0.00001);
}

