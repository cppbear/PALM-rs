// Answer 0

#[test]
fn test_escape_into_with_no_meta_characters() {
    let input = "hello world";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "hello world");
}

#[test]
fn test_escape_into_with_single_meta_character() {
    let input = "a.b";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "a\\.b");
}

#[test]
fn test_escape_into_with_multiple_meta_characters() {
    let input = "a*b+c?d[e]f{g}h|i^j$k";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "a\\*b\\+c\\?d\\[e\\]f\\{g\\}h\\|i\\^j\\$k");
}

#[test]
fn test_escape_into_with_empty_string() {
    let input = "";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "");
}

#[test]
fn test_escape_into_only_meta_characters() {
    let input = ".*+?|()[]{}^$";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "\\.\\*\\+\\?\\|\\(\\)\\[\\]\\{\\}\\^\\$");
}

