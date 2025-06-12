// Answer 0

#[test]
fn test_split_basic() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
    assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_split_multiple_spaces() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("   a      b   c ").collect();
    assert_eq!(fields, vec!["", "a", "b", "c", ""]);
}

#[test]
fn test_split_leading_and_trailing_spaces() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("   a b c   ").collect();
    assert_eq!(fields, vec!["", "a", "b", "c", ""]);
}

#[test]
fn test_split_only_delimiters() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split(" \t \t ").collect();
    assert_eq!(fields, vec!["", "", ""]);
}

#[test]
fn test_split_empty_string() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("").collect();
    assert_eq!(fields, vec![""]);
}

#[test]
fn test_split_no_match() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("abcdefg").collect();
    assert_eq!(fields, vec!["abcdefg"]);
}

#[test]
fn test_split_tabs_and_spaces() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("hello\tworld\ttest ").collect();
    assert_eq!(fields, vec!["hello", "world", "test", ""]);
}

