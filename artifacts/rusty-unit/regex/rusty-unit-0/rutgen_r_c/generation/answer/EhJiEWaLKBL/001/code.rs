// Answer 0

#[test]
fn test_fmt_stateflags_with_match() {
    let state_flags = StateFlags(0b00000001); // Set is_match to true
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);
    assert!(result.is_ok());
    assert!(output.contains("is_match: true"));
    assert!(output.contains("is_word: false"));
    assert!(output.contains("has_empty: false"));
}

#[test]
fn test_fmt_stateflags_with_word() {
    let state_flags = StateFlags(0b00000010); // Set is_word to true
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);
    assert!(result.is_ok());
    assert!(output.contains("is_match: false"));
    assert!(output.contains("is_word: true"));
    assert!(output.contains("has_empty: false"));
}

#[test]
fn test_fmt_stateflags_with_empty() {
    let state_flags = StateFlags(0b00000100); // Set has_empty to true
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);
    assert!(result.is_ok());
    assert!(output.contains("is_match: false"));
    assert!(output.contains("is_word: false"));
    assert!(output.contains("has_empty: true"));
}

#[test]
fn test_fmt_stateflags_with_no_flags() {
    let state_flags = StateFlags(0); // No flags set
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);
    assert!(result.is_ok());
    assert!(output.contains("is_match: false"));
    assert!(output.contains("is_word: false"));
    assert!(output.contains("has_empty: false"));
}

