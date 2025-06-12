// Answer 0

#[test]
fn test_regex_is_match() {
    use regex::Regex;

    let regex = Regex::new(r"\b\w{13}\b").unwrap();

    // Test case with a match
    let text_with_match = "I categorically deny having triskaidekaphobia.";
    assert!(regex.is_match(text_with_match));

    // Test case with no match (less than 13 word characters)
    let text_without_match_short = "I deny having fear.";
    assert!(!regex.is_match(text_without_match_short));

    // Test case with no match (more than 13 word characters)
    let text_without_match_long = "I have a fear of triskaidekaphobias.";
    assert!(!regex.is_match(text_without_match_long));

    // Test case with a match (exactly 13 word characters)
    let text_with_exact_match = "Welcome to the triskaidekaphobia.";
    assert!(regex.is_match(text_with_exact_match));

    // Edge case: empty string (should not match)
    let empty_text = "";
    assert!(!regex.is_match(empty_text));

    // Edge case: string with special characters only (should not match)
    let special_chars_text = "!@#$%^&*()_+";
    assert!(!regex.is_match(special_chars_text));

    // Edge case: string with whitespace only (should not match)
    let whitespace_text = "     ";
    assert!(!regex.is_match(whitespace_text));  
}

