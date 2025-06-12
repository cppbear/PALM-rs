// Answer 0

#[test]
fn test_jaro_identical_strings() {
    let a = "test";
    let b = "test";
    assert!((1.0 - jaro(a, b)).abs() < 0.001);
}

#[test]
fn test_jaro_completely_different_strings() {
    let a = "hello";
    let b = "world";
    assert!((0.0 - jaro(a, b)).abs() < 0.001);
}

#[test]
fn test_jaro_partial_similarity() {
    let a = "Friedrich Nietzsche";
    let b = "Jean-Paul Sartre";
    assert!((0.392 - jaro(a, b)).abs() < 0.001);
}

#[test]
fn test_jaro_empty_strings() {
    let a = "";
    let b = "";
    assert!((1.0 - jaro(a, b)).abs() < 0.001);
}

#[test]
fn test_jaro_one_empty_string() {
    let a = "hello";
    let b = "";
    assert!((0.0 - jaro(a, b)).abs() < 0.001);
}

#[test]
fn test_jaro_one_character_strings() {
    let a = "a";
    let b = "a";
    assert!((1.0 - jaro(a, b)).abs() < 0.001);
    
    let c = "a";
    let d = "b";
    assert!((0.0 - jaro(c, d)).abs() < 0.001);
}

