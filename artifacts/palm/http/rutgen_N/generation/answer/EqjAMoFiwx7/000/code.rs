// Answer 0

#[test]
fn test_valid_authority() {
    let authority = Authority::from_static("example.com");
    assert_eq!(authority.host(), "example.com");
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_empty_authority() {
    Authority::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid authority")]
fn test_invalid_character_authority() {
    Authority::from_static("example.com:80#fragment");
}

