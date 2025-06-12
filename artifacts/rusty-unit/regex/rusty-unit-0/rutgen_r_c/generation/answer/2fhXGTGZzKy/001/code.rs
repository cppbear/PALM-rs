// Answer 0

#[test]
fn test_escape_into_with_meta_characters() {
    let mut buf = String::new();
    let input = "Hello (world) [test] + regex";
    escape_into(input, &mut buf);
    assert_eq!(buf, "Hello \\(world\\) \\[test\\] \\+ regex");
}

#[test]
fn test_escape_into_with_no_meta_characters() {
    let mut buf = String::new();
    let input = "Just a simple string";
    escape_into(input, &mut buf);
    assert_eq!(buf, "Just a simple string");
}

#[test]
fn test_escape_into_with_only_meta_characters() {
    let mut buf = String::new();
    let input = ".+*?|()[]{}^$#&~-";
    escape_into(input, &mut buf);
    assert_eq!(buf, "\\.\\+\\*\\?\\|\\(\\)\\[\\]\\{\\}\\^\\$\\#\\&\\-\\~");
}

#[test]
fn test_escape_into_with_empty_string() {
    let mut buf = String::new();
    let input = "";
    escape_into(input, &mut buf);
    assert_eq!(buf, "");
}

#[test]
fn test_escape_into_with_mixed_characters() {
    let mut buf = String::new();
    let input = "Hello \\ world with $pecial characters!";
    escape_into(input, &mut buf);
    assert_eq!(buf, "Hello \\\\ world with \\$pecial characters!");
}

