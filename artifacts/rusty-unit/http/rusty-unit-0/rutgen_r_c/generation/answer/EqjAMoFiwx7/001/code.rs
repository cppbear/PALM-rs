// Answer 0

#[test]
fn test_from_static_valid_authority() {
    let authority = Authority::from_static("example.com");
    assert_eq!(authority.host(), "example.com");
}

#[should_panic(expected = "static str is not valid authority")]
#[test]
fn test_from_static_empty_string() {
    Authority::from_static("");
}

#[should_panic(expected = "static str is not valid authority")]
#[test]
fn test_from_static_invalid_characters() {
    Authority::from_static("example.com:80/invalid");
} 

#[test]
fn test_from_static_with_valid_authority_with_no_port() {
    let authority = Authority::from_static("localhost");
    assert_eq!(authority.host(), "localhost");
}

#[should_panic(expected = "static str is not valid authority")]
#[test]
fn test_from_static_invalid_authority_with_whitespace() {
    Authority::from_static("invalid authority");
}

