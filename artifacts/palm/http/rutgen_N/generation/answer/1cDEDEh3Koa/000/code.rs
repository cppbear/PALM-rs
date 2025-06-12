// Answer 0

#[derive(Debug)]
struct StatusCode(u16);

#[derive(Debug)]
struct InvalidStatusCode;

impl InvalidStatusCode {
    fn new() -> Self {
        InvalidStatusCode
    }
}

use std::num::NonZeroU16;

fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
    if src.len() != 3 {
        return Err(InvalidStatusCode::new());
    }

    let a = src[0].wrapping_sub(b'0') as u16;
    let b = src[1].wrapping_sub(b'0') as u16;
    let c = src[2].wrapping_sub(b'0') as u16;

    if a == 0 || a > 9 || b > 9 || c > 9 {
        return Err(InvalidStatusCode::new());
    }

    let status = (a * 100) + (b * 10) + c;
    NonZeroU16::new(status)
        .map(StatusCode)
        .ok_or_else(InvalidStatusCode::new)
}

#[test]
fn test_from_bytes_valid() {
    let input = b"200";
    let result = from_bytes(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 200);
}

#[test]
fn test_from_bytes_invalid_length() {
    let input = b"20";
    let result = from_bytes(input);
    assert!(result.is_err());

    let input = b"2000";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_characters() {
    let input = b"0A0";
    let result = from_bytes(input);
    assert!(result.is_err());

    let input = b"300";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_zero_first_digit() {
    let input = b"000";
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_out_of_bounds() {
    let input = b"999";
    let result = from_bytes(input);
    assert!(result.is_err());
}

