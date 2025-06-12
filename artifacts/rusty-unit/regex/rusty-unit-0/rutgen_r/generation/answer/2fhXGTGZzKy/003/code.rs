// Answer 0

#[test]
fn test_escape_into_no_meta_characters() {
    let input = "abc";
    let mut buf = String::new();
    escape_into(input, &mut buf);
    assert_eq!(buf, "abc");
}

#[test]
fn test_escape_into_single_meta_character() {
    let input = "a.b";
    let mut buf = String::new();
    escape_into(input, &mut buf);
    assert_eq!(buf, "a\\.b");
}

#[test]
fn test_escape_into_multiple_meta_characters() {
    let input = "a*b+c?d";
    let mut buf = String::new();
    escape_into(input, &mut buf);
    assert_eq!(buf, "a\\*b\\+c\\?d");
}

#[test]
fn test_escape_into_all_meta_characters() {
    let input = ".^$*+?{}[]\\|()";
    let mut buf = String::new();
    escape_into(input, &mut buf);
    assert_eq!(buf, "\\.\\^\\$\\*\\+\\?\\{\\}\\[\\]\\\\\\|\\(\\)");
}

#[test]
fn test_escape_into_empty_string() {
    let input = "";
    let mut buf = String::new();
    escape_into(input, &mut buf);
    assert_eq!(buf, "");
}

