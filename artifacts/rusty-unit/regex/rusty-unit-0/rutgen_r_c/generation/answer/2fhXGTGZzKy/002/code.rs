// Answer 0

#[test]
fn test_escape_into_with_no_meta_characters() {
    let mut buf = String::new();
    escape_into("abcd", &mut buf);
    assert_eq!(buf, "abcd");
}

#[test]
fn test_escape_into_with_all_meta_characters() {
    let mut buf = String::new();
    escape_into("\\.+*?()|[]{}^$&#~-", &mut buf);
    assert_eq!(buf, "\\\\.+*?()|[]{}^$&#~-");
}

#[test]
fn test_escape_into_with_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
    assert_eq!(buf, "");
}

#[test]
fn test_escape_into_with_non_meta_characters_only() {
    let mut buf = String::new();
    escape_into("hello, world!", &mut buf);
    assert_eq!(buf, "hello, world!");
}

#[test]
fn test_escape_into_with_mixed_characters() {
    let mut buf = String::new();
    escape_into("abc*def?ghi(jkl)mno", &mut buf);
    assert_eq!(buf, "abc\\*def\\?ghi\\(jkl\\)mno");
}

#[test]
fn test_escape_into_with_all_characters() {
    let mut buf = String::new();
    escape_into("abc\\\\def.+*?()|[]{}^$&#~-xyz", &mut buf);
    assert_eq!(buf, "abc\\\\def\\.+*?()|[]{}^$&#~-xyz");
}

