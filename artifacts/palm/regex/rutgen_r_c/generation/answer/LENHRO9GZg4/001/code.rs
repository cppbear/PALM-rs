// Answer 0

#[test]
fn test_split_basic() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
    assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_split_multiple_spaces() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a   b   c").collect();
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_split_leading_trailing_spaces() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("   a b   ").collect();
    assert_eq!(fields, vec!["", "a", "b", ""]);
}

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("").collect();
    assert_eq!(fields, vec![""]);
}

#[test]
fn test_split_no_matches() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("abc").collect();
    assert_eq!(fields, vec!["abc"]);
}

#[test]
fn test_split_only_delimiters() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("    ").collect();
    assert_eq!(fields, vec!["", "", "", "", ""]);
}

