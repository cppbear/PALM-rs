// Answer 0

#[test]
fn test_is_match_positive() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "I categorically deny having triskaidekaphobia.";
    let regex = Regex::new(pattern).unwrap();
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_negative() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "No long words here.";
    let regex = Regex::new(pattern).unwrap();
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_empty_string() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "";
    let regex = Regex::new(pattern).unwrap();
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_boundary_length() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "abcdefghijklm"; // exactly 13 characters
    let regex = Regex::new(pattern).unwrap();
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_boundary_length_too_short() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "abcdefghijkl"; // 12 characters
    let regex = Regex::new(pattern).unwrap();
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_boundary_length_too_long() {
    use regex::Regex;
    let pattern = r"\b\w{13}\b";
    let text = "abcdefghijklmnop"; // 14 characters
    let regex = Regex::new(pattern).unwrap();
    assert!(!regex.is_match(text));
}

