// Answer 0

#[test]
fn test_escape_into_no_meta_characters() {
    let input = "abc123";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "abc123");
}

#[test]
fn test_escape_into_with_meta_characters() {
    let input = "a.b*?+|^$()[]{}";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "a\\.b\\*\\?\\+\\|\\^\\$\\(\\)\\[\\]\\{\\}");
}

#[test]
fn test_escape_into_empty_string() {
    let input = "";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "");
}

#[test]
fn test_escape_into_single_character_meta() {
    let input = ".";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "\\.");
}

#[test]
fn test_escape_into_single_character_non_meta() {
    let input = "a";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "a");
}

#[test]
fn test_escape_into_all_types_of_meta() {
    let input = r"abc\def\ghi\jkl";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, r"abc\\def\\ghi\\jkl");
}

