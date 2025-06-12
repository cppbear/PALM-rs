// Answer 0

#[test]
fn test_is_match_empty_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("");
}

#[test]
fn test_is_match_single_character_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("a");
}

#[test]
fn test_is_match_multiple_character_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("hello world");
}

#[test]
fn test_is_match_exactly_13_characters() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("triskaidekaphobia");
}

#[test]
fn test_is_match_more_than_13_characters() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("thisstringhas14characters");
}

#[test]
fn test_is_match_less_than_13_characters() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("shortword");
}

#[test]
fn test_is_match_special_characters() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    regex.is_match("!@#$%^&*()_+ {}[]");
}

#[test]
fn test_is_match_very_long_string() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let long_string = "a".repeat(1000);
    regex.is_match(&long_string);
}

