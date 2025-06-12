// Answer 0

#[test]
fn test_splitn_with_limit_greater_than_splits() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hey! How are you?", 5).collect();
    assert_eq!(fields, vec!("Hey", "How", "are", "you?"));
}

#[test]
fn test_splitn_with_limit_equal_to_splits() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hello, World! How are you?", 4).collect();
    assert_eq!(fields, vec!("Hello", "World", "How", "are you?"));
}

#[test]
fn test_splitn_with_limit_one() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Rust is fun", 1).collect();
    assert_eq!(fields, vec!("Rust is fun"));
}

#[test]
fn test_splitn_with_limit_zero() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("This should not split", 0).collect();
    assert_eq!(fields, vec!());
}

#[test]
fn test_splitn_with_empty_string() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("", 3).collect();
    assert_eq!(fields, vec!(""));
}

#[test]
fn test_splitn_matches_only_punctuation() {
    use regex::Regex;

    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Word1,Word2!Word3.Word4?", 4).collect();
    assert_eq!(fields, vec!("Word1", "Word2", "Word3", "Word4?"));
}

