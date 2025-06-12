// Answer 0

#[test]
fn test_from_static_valid_authority() {
    let authority = Authority::from_static("example.com");
    assert_eq!(authority.host(), "example.com");
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_from_static_empty_string() {
    Authority::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_from_static_invalid_characters() {
    Authority::from_static("example.com:80/"); // Invalid because of '/'
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_from_static_invalid_start_character() {
    Authority::from_static(".example.com"); // Invalid because starts with a '.'
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_from_static_invalid_end_character() {
    Authority::from_static("example.com."); // Invalid because ends with a '.'
}

#[test]
fn test_from_static_valid_with_ipv4() {
    let authority = Authority::from_static("192.168.1.1");
    assert_eq!(authority.host(), "192.168.1.1");
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_from_static_ipv4_invalid_last_segment() {
    Authority::from_static("192.168.1.256"); // Invalid because of segment > 255
}

