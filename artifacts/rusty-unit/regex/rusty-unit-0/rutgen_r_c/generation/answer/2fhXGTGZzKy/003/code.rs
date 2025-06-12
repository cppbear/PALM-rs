// Answer 0

#[test]
fn test_escape_into_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
    assert_eq!(buf, "");
}

#[test]
fn test_escape_into_no_meta_characters() {
    let mut buf = String::new();
    escape_into("abc", &mut buf);
    assert_eq!(buf, "abc");
}

#[test]
fn test_escape_into_single_meta_character() {
    let mut buf = String::new();
    escape_into(".", &mut buf);
    assert_eq!(buf, "\\.");
}

#[test]
fn test_escape_into_multiple_meta_characters() {
    let mut buf = String::new();
    escape_into("a.b+c*?", &mut buf);
    assert_eq!(buf, "a\\.b\\+c\\*\\?");
}

#[test]
fn test_escape_into_edge_case_special_characters() {
    let mut buf = String::new();
    escape_into("abc[]{}()^$|", &mut buf);
    assert_eq!(buf, "abc\\[\\{\\}\\(\\)\\^\\$\\|");
}

#[test]
fn test_escape_into_only_meta_characters() {
    let mut buf = String::new();
    escape_into(".*?+|(){}", &mut buf);
    assert_eq!(buf, "\\.\\*\\?\\+\\|\\(\\)\\{\\}");
}

