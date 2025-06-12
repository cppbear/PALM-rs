// Answer 0

#[test]
fn test_from_static_valid_ascii() {
    let val = from_static("hello");
    assert_eq!(val.inner.as_ref(), b"hello");
    assert_eq!(val.is_sensitive, false);
}

#[test]
fn test_from_static_empty_string() {
    let val = from_static("");
    assert_eq!(val.inner.as_ref(), b"");
    assert_eq!(val.is_sensitive, false);
}

#[should_panic(expected = "any use of this value will cause an error")]
fn test_from_static_invalid_character() {
    let _val = from_static("hello ж");
}

#[test]
fn test_from_static_boundary_condition() {
    let val = from_static(" ");
    assert_eq!(val.inner.as_ref(), b" ");
    assert_eq!(val.is_sensitive, false);
}

#[should_panic(expected = "any use of this value will cause an error")]
fn test_from_static_non_visible_ascii() {
    let _val = from_static("\x00"); // control character
}

#[test]
fn test_from_static_all_visible_ascii() {
    let val = from_static("~!@#$%^&*()-_=+[]{}|;:',.<>?/");
    assert_eq!(val.inner.as_ref(), b"~!@#$%^&*()-_=+[]{}|;:',.<>?/");
    assert_eq!(val.is_sensitive, false);
}

