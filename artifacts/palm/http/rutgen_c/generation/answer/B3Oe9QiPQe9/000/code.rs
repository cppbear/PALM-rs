// Answer 0

#[test]
fn test_try_from_valid_code() {
    let input = &b"200"[..];
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 200);
}

#[test]
fn test_try_from_invalid_length() {
    let input = &b"20"[..]; // length less than 3
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_character() {
    let input = &b"20A"[..]; // non-numeric character
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_out_of_bounds() {
    let input = &b"600"[..]; // out of valid HTTP status code range
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

