// Answer 0

#[test]
fn test_is_valid_with_printable_character() {
    let result = is_valid(b'A');
    assert!(result);
}

#[test]
fn test_is_valid_with_control_character() {
    let result = is_valid(b'\n');
    assert!(!result);
}

#[test]
fn test_is_valid_with_backspace_character() {
    let result = is_valid(b'\x08'); // Backspace
    assert!(!result);
}

#[test]
fn test_is_valid_with_tab_character() {
    let result = is_valid(b'\t');
    assert!(result);
}

#[test]
fn test_is_valid_with_delete_character() {
    let result = is_valid(127);
    assert!(!result);
}

#[test]
fn test_is_valid_with_non_printable_character() {
    let result = is_valid(31); // Unit Separator
    assert!(!result);
}

#[test]
fn test_is_valid_with_min_value() {
    let result = is_valid(0);
    assert!(!result);
}

#[test]
fn test_is_valid_with_max_value() {
    let result = is_valid(255);
    assert!(result);
}

