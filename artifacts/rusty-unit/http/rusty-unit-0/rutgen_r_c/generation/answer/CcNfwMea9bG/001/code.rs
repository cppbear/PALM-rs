// Answer 0

#[test]
fn test_to_str_all_visible_ascii() {
    let val = HeaderValue::from_static("visible");
    assert_eq!(val.to_str().unwrap(), "visible");
}

#[test]
fn test_to_str_empty_string() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.to_str().unwrap(), "");
}

#[test]
#[should_panic]
fn test_to_str_non_visible_ascii() {
    let val = HeaderValue::from_static("invalid\0character"); // null byte is non-visible ASCII
    val.to_str().unwrap(); // This should panic
}

#[test]
fn test_to_str_with_tab() {
    let val = HeaderValue::from_static("valid\tcharacter");
    assert_eq!(val.to_str().unwrap(), "valid\tcharacter");
}

#[test]
#[should_panic]
fn test_to_str_with_non_visible_ascii() {
    let val = HeaderValue::from_static("not valid!\n"); // newline is non-visible ASCII
    val.to_str().unwrap(); // This should panic
}

