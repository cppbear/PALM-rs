// Answer 0

#[test]
fn test_split_basic() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
    assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_split_multiple_delimiters() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("   a   b    c   ").collect();
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_split_empty_string() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("").collect();
    assert_eq!(fields, vec![""]);
}

#[test]
fn test_split_only_delimiters() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("    \t   ").collect();
    assert_eq!(fields, vec![""]);
}

#[test]
fn test_split_no_delimiters() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("abcdef").collect();
    assert_eq!(fields, vec!["abcdef"]);
}

#[test]
fn test_split_leading_and_trailing_delimiters() {
    use regex::Regex;

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("   abc def   ").collect();
    assert_eq!(fields, vec!["abc", "def"]);
}

