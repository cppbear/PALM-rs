// Answer 0

#[test]
fn test_is_match_basic() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "I categorically deny having triskaidekaphobia.";
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_no_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "I deny having a fear of long words.";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_empty_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "";
    assert!(!regex.is_match(text));
}

#[test]
fn test_is_match_edge_case() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "abcdefghijklm"; // 13 characters
    assert!(regex.is_match(text));
}

#[test]
fn test_is_match_multiple_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text = "I categorically deny triskaidekaphobia but also supercalifragilisticexpialidocious.";
    assert!(regex.is_match(text));
}

