// Answer 0

#[test]
fn test_splitn_with_non_zero_limit() {
    let re = regex::Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hey! How are you?", 3).collect();
    assert_eq!(fields, vec!("Hey", "How", "are you?"));
}

#[test]
fn test_splitn_with_zero_limit() {
    let re = regex::Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hey! How are you?", 0).collect();
    assert_eq!(fields, vec![]);
}

#[test]
fn test_splitn_with_limit_greater_than_words() {
    let re = regex::Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("A B C D", 10).collect();
    assert_eq!(fields, vec!("A", "B", "C", "D"));
}

#[test]
fn test_splitn_with_single_word() {
    let re = regex::Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hello", 2).collect();
    assert_eq!(fields, vec!("Hello"));
}

#[test]
fn test_splitn_with_empty_string() {
    let re = regex::Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("", 5).collect();
    assert_eq!(fields, vec!( ""));
}

