// Answer 0

#[test]
fn test_simple_fold_existing_mapping() {
    let codepoint = 'a';
    let result = simple_fold(codepoint);
    assert!(result.is_ok());
    if let Ok(iter) = result {
        assert!(iter.0.clone().count() > 0); // Expecting it to have an equivalence class
    }
}

#[test]
fn test_simple_fold_no_mapping() {
    let codepoint = '𝒸'; // A character that does not have a mapping
    let result = simple_fold(codepoint);
    assert!(result.is_err());
    if let Err(Some(next)) = result {
        assert_eq!(next, '𝒸'); // Assert it returns the next scalar value
    }
}

#[test]
fn test_simple_fold_empty_class() {
    let codepoint = 'Ā'; // This should not map, expect the next scalar
    let result = simple_fold(codepoint);
    assert!(result.is_err());
    if let Err(Some(next)) = result {
        assert!(next.is_alphanumeric()); // Checking the next scalar is alphanumeric
    }
}

#[test]
fn test_simple_fold_high_char() {
    let codepoint = '\u{10FFFF}'; // The highest scalar value
    let result = simple_fold(codepoint);
    assert!(result.is_err());
    if let Err(None) = result {
        // Expecting None because there are no characters above the highest scalar
    }
}

