// Answer 0

#[test]
fn test_from_static_empty_string() {
    let val = HeaderValue::from_static("");
}

#[test]
fn test_from_static_valid_string() {
    let val = HeaderValue::from_static("hello");
}

#[test]
#[should_panic]
fn test_from_static_invalid_character_visible() {
    let val = HeaderValue::from_static("hello\x7f");
}

#[test]
#[should_panic]
fn test_from_static_invalid_character_invisible() {
    let val = HeaderValue::from_static("hello\x1f");
}

#[test]
#[should_panic]
fn test_from_static_invalid_non_ascii() {
    let val = HeaderValue::from_static("hello ват");
}

#[test]
fn test_from_static_valid_tab_character() {
    let val = HeaderValue::from_static("valid\t");
}

#[test]
fn test_from_static_valid_space_character() {
    let val = HeaderValue::from_static(" ");
}

#[test]
fn test_from_static_another_valid_string() {
    let val = HeaderValue::from_static("visible");
}

#[test]
#[should_panic]
fn test_from_static_invalid_non_visible_character() {
    let val = HeaderValue::from_static("invalid\x08");
}

