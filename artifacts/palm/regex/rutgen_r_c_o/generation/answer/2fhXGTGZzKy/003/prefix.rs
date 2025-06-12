// Answer 0

#[test]
fn test_escape_into_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
}

#[test]
fn test_escape_into_no_meta_characters() {
    let mut buf = String::new();
    escape_into("hello", &mut buf);
}

#[test]
fn test_escape_into_all_meta_characters() {
    let mut buf = String::new();
    escape_into("\\.+*?()|[]{}^$&#-~", &mut buf);
}

#[test]
fn test_escape_into_mixed_characters() {
    let mut buf = String::new();
    escape_into("test\\*string with (meta) characters $", &mut buf);
}

#[test]
fn test_escape_into_only_meta_characters() {
    let mut buf = String::new();
    escape_into("\\" , &mut buf);
}

