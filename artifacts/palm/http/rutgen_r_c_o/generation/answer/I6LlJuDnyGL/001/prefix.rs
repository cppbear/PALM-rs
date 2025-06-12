// Answer 0

#[test]
#[should_panic]
fn test_from_static_empty_string() {
    Uri::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_single_invalid_char() {
    Uri::from_static("~");
}

#[test]
#[should_panic]
fn test_from_static_single_invalid_char_percent() {
    Uri::from_static("%");
}

#[test]
#[should_panic]
fn test_from_static_exceed_max_length() {
    let long_string = "http://example.com/".to_string() + &"a".repeat(65534);
    Uri::from_static(&long_string);
}

#[test]
#[should_panic]
fn test_from_static_invalid_uri_pattern() {
    Uri::from_static("ftp://:badhost/");
}

