// Answer 0

#[test]
fn test_try_from_valid_code() {
    let input: &[u8] = b"200";
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_ok());
    if let Ok(status_code) = result {
        assert_eq!(status_code.as_u16(), 200);
        assert_eq!(status_code.as_str(), "OK");
        assert!(status_code.is_success());
    }
}

#[test]
fn test_try_from_invalid_length() {
    let input: &[u8] = b"20"; // Length less than 3
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_invalid_character() {
    let input: &[u8] = b"2a0"; // Contains an invalid character
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_out_of_bounds() {
    let input: &[u8] = b"999"; // Out of bounds status code
    let result: Result<StatusCode, InvalidStatusCode> = StatusCode::try_from(input);
    assert!(result.is_err());
}

