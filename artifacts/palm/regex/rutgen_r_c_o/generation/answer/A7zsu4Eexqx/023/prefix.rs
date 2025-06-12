// Answer 0

#[test]
fn test_is_valid_cap_letter_with_underscore() {
    let b = &b'_'; 
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_digit() {
    let b = &b'3'; 
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase() {
    let b = &b'g'; 
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase() {
    let b = &b'K'; 
    is_valid_cap_letter(b);
}

#[test]
fn test_is_valid_cap_letter_with_special_character() {
    let b = &b'@'; 
    is_valid_cap_letter(b);
}

