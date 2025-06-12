// Answer 0

#[test]
fn test_replacen_no_expansion_and_no_matches() {
    let regex = Regex::new(r"\d+").unwrap();
    let text: &[u8] = b"abc";
    let replacement = b"xyz";
    
    let result = regex.replacen(text, 0, replacement);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_with_one_match() {
    let regex = Regex::new(r"\d+").unwrap();
    let text: &[u8] = b"abc 123 and 456";
    let replacement = b"xyz";

    let result = regex.replacen(text, 1, replacement);
    let expected = b"abc xyz and 456";
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

#[test]
fn test_replacen_no_expansion_with_multiple_matches_limit_zero() {
    let regex = Regex::new(r"\d+").unwrap();
    let text: &[u8] = b"123 and 456 and 789";
    let replacement = b"xyz";

    let result = regex.replacen(text, 0, replacement);
    let expected = b"xyz and xyz and xyz";
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

#[test]
#[should_panic]
fn test_replacen_captures_get_unwrap_panic() {
    let regex = Regex::new(r"(?P<group>\d+)").unwrap();
    let text: &[u8] = b"abc";
    let replacement = b"xyz";
    
    // This will trigger the panic on cap.get(0).unwrap()
    let _result = regex.replacen(text, 1, replacement);
}

#[test]
#[should_panic]
fn test_replacen_text_last_match_panic() {
    let regex = Regex::new(r"\w+").unwrap();
    let text: &[u8] = b"abc";
    let replacement = b"xyz";
    
    // This may panic due to incorrect slicing
    // Simulating not having any match and trying to slice
    let _result = regex.replacen(text, 1, replacement);
}

#[test]
fn test_replacen_limit_greater_than_zero_no_matches() {
    let regex = Regex::new(r"\d+").unwrap();
    let text: &[u8] = b"abc";
    let replacement = b"xyz";

    let result = regex.replacen(text, 1, replacement);
    assert_eq!(result, Cow::Borrowed(text));
}

