// Answer 0

#[test]
fn test_uppercase_generates_char_in_range() {
    let generated_char = fastrand::uppercase();
    assert!(generated_char.is_ascii_uppercase(), "Generated character is not uppercase.");
    assert!(generated_char >= 'A' && generated_char <= 'Z', "Generated character is out of range A-Z.");
}

#[test]
fn test_uppercase_edge_cases() {
    let generated_char = fastrand::uppercase();
    assert!(generated_char >= 'A' && generated_char <= 'Z', "Generated character does not fall within edge cases A-Z.");
}

