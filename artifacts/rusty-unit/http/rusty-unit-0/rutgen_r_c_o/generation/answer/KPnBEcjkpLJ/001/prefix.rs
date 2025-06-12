// Answer 0

#[test]
fn test_authority_valid_string() {
    let builder = Builder::new();
    let uri = builder.authority("example.com").build().unwrap();
}

#[test]
fn test_authority_with_user_info() {
    let builder = Builder::new();
    let uri = builder.authority("user@host:8080").build().unwrap();
}

#[test]
fn test_authority_empty_string() {
    let builder = Builder::new();
    let result = builder.authority("").build();
    assert!(result.is_err());
}

#[test]
fn test_authority_overly_long_string() {
    let long_auth = "a".repeat(256);
    let builder = Builder::new();
    let result = builder.authority(long_auth).build();
    assert!(result.is_err());
}

#[test]
fn test_authority_invalid_format_with_scheme() {
    let builder = Builder::new();
    let result = builder.authority("http://invalid").build();
    assert!(result.is_err());
}

#[test]
fn test_authority_invalid_format_no_host() {
    let builder = Builder::new();
    let result = builder.authority("invalid:host").build();
    assert!(result.is_err());
}

#[test]
fn test_authority_with_special_characters() {
    let builder = Builder::new();
    let uri = builder.authority("user:password@host").build().unwrap();
}

#[test]
fn test_authority_null_input() {
    let builder = Builder::new();
    let result = builder.authority(None::<&str>).build();
    assert!(result.is_err());
}

