// Answer 0

#[derive(Debug, PartialEq)]
struct StatusCode(u16);

#[derive(Debug)]
struct InvalidStatusCode;

impl InvalidStatusCode {
    fn new() -> Self {
        InvalidStatusCode
    }
}

impl StatusCode {
    fn from_u16(src: u16) -> Result<Self, InvalidStatusCode> {
        if !(100..1000).contains(&src) {
            return Err(InvalidStatusCode::new());
        }

        std::num::NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }
}

#[test]
fn test_from_u16_valid() {
    let ok = StatusCode::from_u16(200).unwrap();
    assert_eq!(ok, StatusCode(200));
}

#[test]
fn test_from_u16_too_low() {
    let err = StatusCode::from_u16(99);
    assert!(err.is_err());
}

#[test]
fn test_from_u16_too_high() {
    let err = StatusCode::from_u16(1000);
    assert!(err.is_err());
}

#[test]
fn test_from_u16_boundary_low() {
    let ok = StatusCode::from_u16(100).unwrap();
    assert_eq!(ok, StatusCode(100));
}

#[test]
fn test_from_u16_boundary_high() {
    let err = StatusCode::from_u16(999);
    assert!(err.is_ok());
    assert_eq!(err.unwrap(), StatusCode(999));
}

