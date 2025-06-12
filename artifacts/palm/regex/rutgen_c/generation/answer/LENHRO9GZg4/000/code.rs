// Answer 0

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("").collect();
    assert_eq!(fields, vec![""]);
}

#[test]
fn test_split_no_match() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("hello").collect();
    assert_eq!(fields, vec!["hello"]);
}

#[test]
fn test_split_single_delimiter() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a b").collect();
    assert_eq!(fields, vec!["a", "b"]);
}

#[test]
fn test_split_multiple_consecutive_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a  b  c").collect();
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_split_leading_and_trailing_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("  a b  ").collect();
    assert_eq!(fields, vec!["", "a", "b", ""]);
}

#[test]
fn test_split_only_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("    ").collect();
    assert_eq!(fields, vec!["", "", "", "", ""]);
}

