// Answer 0

#[test]
fn test_to_str_with_visible_ascii() {
    let val = HeaderValue::from_static("visible");
    assert_eq!(val.to_str().unwrap(), "visible");
}

#[test]
fn test_to_str_with_non_visible_ascii() {
    let val = HeaderValue::from_static("non-visible\x00");
    assert!(val.to_str().is_err());
}

#[test]
fn test_to_str_with_mixed_visible_and_non_visible() {
    let val = HeaderValue::from_static("valid\x00string");
    assert!(val.to_str().is_err());
}

#[test]
fn test_to_str_empty_string() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.to_str().unwrap(), "");
}

#[test]
fn test_to_str_single_non_visible_ascii() {
    let val = HeaderValue::from_static("\x01");
    assert!(val.to_str().is_err());
}

