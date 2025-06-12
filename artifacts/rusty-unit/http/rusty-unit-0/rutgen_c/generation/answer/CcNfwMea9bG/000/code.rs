// Answer 0

#[test]
fn test_to_str_visible_ascii() {
    let val = HeaderValue::from_static("hello");
    assert_eq!(val.to_str().unwrap(), "hello");
}

#[test]
#[should_panic]
fn test_to_str_non_visible_ascii() {
    let val = HeaderValue::from_static("hello\x00world");
    val.to_str().unwrap();
}

#[test]
fn test_to_str_only_tab() {
    let val = HeaderValue::from_static("\thello");
    assert_eq!(val.to_str().unwrap(), "\thello");
}

#[test]
fn test_to_str_empty_string() {
    let val = HeaderValue::from_static("");
    assert_eq!(val.to_str().unwrap(), "");
}

