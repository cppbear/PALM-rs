// Answer 0

#[test]
fn test_collect_str_empty() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_normal_string() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "Hello, World!";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_special_characters() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "😊";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_long_string() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "A very long string that exceeds normal length limits ..........................................";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_numeric_string() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "12345";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_whitespace_string() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "     ";
    formatter.collect_str(&value);
}

#[test]
fn test_collect_str_newline_string() {
    let mut formatter = std::fmt::Formatter::new();
    let value = "Line 1\nLine 2";
    formatter.collect_str(&value);
}

