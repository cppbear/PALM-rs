// Answer 0

#[test]
fn test_from_bytes_valid_code() {
    let input: &[u8] = b"200";
    let result = from_bytes(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 200);
}

#[test]
fn test_from_bytes_invalid_length() {
    let input: &[u8] = b"20"; // too short
    let result = from_bytes(input);
    assert!(result.is_err());

    let input: &[u8] = b"2000"; // too long
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_leading_zero() {
    let input: &[u8] = b"000"; // a == 0
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_out_of_bounds() {
    let input: &[u8] = b"300"; // a > 9
    let result = from_bytes(input);
    assert!(result.is_err());

    let input: &[u8] = b"20a"; // b > 9
    let result = from_bytes(input);
    assert!(result.is_err());

    let input: &[u8] = b"21 "; // c > 9
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_valid_boundaries() {
    let input: &[u8] = b"999"; // upper bound
    let result = from_bytes(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 999);
}

