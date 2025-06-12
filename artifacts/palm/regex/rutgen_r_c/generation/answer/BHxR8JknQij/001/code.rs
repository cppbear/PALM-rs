// Answer 0

#[test]
fn test_splitn_with_valid_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let result: Vec<&str> = re.splitn("Hey! How are you?", 3).collect();
    assert_eq!(result, vec!["Hey", "How", "are you?"]);
}

#[test]
fn test_splitn_with_zero_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let result: Vec<&str> = re.splitn("Hey! How are you?", 0).collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_splitn_with_one_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let result: Vec<&str> = re.splitn("Hey! How are you?", 1).collect();
    assert_eq!(result, vec!["Hey! How are you?"]);
}

#[test]
fn test_splitn_with_large_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let result: Vec<&str> = re.splitn("One, Two, Three, Four", 10).collect();
    assert_eq!(result, vec!["One", "Two", "Three", "Four"]);
}

#[test]
fn test_splitn_with_empty_string() {
    let re = Regex::new(r"\W+").unwrap();
    let result: Vec<&str> = re.splitn("", 3).collect();
    assert_eq!(result, vec![""]);
}

#[test]
fn test_splitn_on_string_with_no_matches() {
    let re = Regex::new(r"\d+").unwrap();
    let result: Vec<&str> = re.splitn("HelloWorld", 3).collect();
    assert_eq!(result, vec!["HelloWorld"]);
}

