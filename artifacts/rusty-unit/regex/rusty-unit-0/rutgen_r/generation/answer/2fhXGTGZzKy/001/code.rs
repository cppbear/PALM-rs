// Answer 0

#[test]
fn test_escape_into_no_meta_characters() {
    let text = "abc";
    let mut buf = String::new();
    escape_into(text, &mut buf);
    assert_eq!(buf, "abc");
}

#[test]
fn test_escape_into_with_meta_characters() {
    let text = "a.b[c]*xyz";
    let mut buf = String::new();
    escape_into(text, &mut buf);
    assert_eq!(buf, "a\\.b\\[c\\]\\*xyz");
}

#[test]
fn test_escape_into_only_meta_characters() {
    let text = ".*+?^$|(){}";
    let mut buf = String::new();
    escape_into(text, &mut buf);
    assert_eq!(buf, "\\.\\*\\+\\?\\^\\$\\|\\(\\)\\{\\}");
}

#[test]
fn test_escape_into_empty_string() {
    let text = "";
    let mut buf = String::new();
    escape_into(text, &mut buf);
    assert_eq!(buf, "");
}

#[test]
fn test_escape_into_mixed_characters() {
    let text = "hello? world!";
    let mut buf = String::new();
    escape_into(text, &mut buf);
    assert_eq!(buf, "hello\\? world!");
}

