// Answer 0

#[test]
fn test_to_str_visible_ascii() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.to_str().unwrap(), "hello");

    let val_with_tabs = HeaderValue::from_static("hello\tworld");
    assert_eq!(val_with_tabs.to_str().unwrap(), "hello\tworld");

    let val_with_spaces = HeaderValue::from_static("hello world");
    assert_eq!(val_with_spaces.to_str().unwrap(), "hello world");

    let val_with_only_visible_ascii = HeaderValue::from_static("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    assert_eq!(val_with_only_visible_ascii.to_str().unwrap(), "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
}

#[test]
#[should_panic]
fn test_to_str_non_visible_ascii() {
    let val_with_non_visible = HeaderValue::from_static("hello\x01world");
    let _ = val_with_non_visible.to_str(); // This should trigger a panic due to non-visible ASCII
}

#[test]
#[should_panic]
fn test_to_str_with_newline() {
    let val_with_newline = HeaderValue::from_static("hello\nworld");
    let _ = val_with_newline.to_str(); // This should trigger a panic due to non-visible ASCII
}

#[test]
fn test_to_str_empty() {
    let empty_val = HeaderValue::from_static("");
    assert_eq!(empty_val.to_str().unwrap(), "");
}

#[test]
fn test_to_str_sensitive_value() {
    let mut sensitive_val = HeaderValue::from_static("sensitive_data");
    sensitive_val.set_sensitive(true);
    assert_eq!(sensitive_val.to_str().unwrap(), "sensitive_data");
}

