// Answer 0

#[test]
fn test_splitn_with_positive_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hey! How are you?", 3).collect();
    assert_eq!(fields, vec!("Hey", "How", "are you?"));
}

#[test]
fn test_splitn_with_zero_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("Hey! How are you?", 0).collect();
    assert_eq!(fields, Vec::<&str>::new());
}

#[test]
fn test_splitn_with_limit_greater_than_splits() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("One, two, three", 5).collect();
    assert_eq!(fields, vec!("One", "two", "three"));
}

#[test]
fn test_splitn_with_no_matches() {
    let re = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = re.splitn("TwoWords", 2).collect();
    assert_eq!(fields, vec!("TwoWords"));
}

