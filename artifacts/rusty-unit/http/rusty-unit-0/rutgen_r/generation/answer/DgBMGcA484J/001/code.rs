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

impl StatusCode {
    fn from_u16(src: u16) -> Result<Self, InvalidStatusCode> {
        if !(100..1000).contains(&src) {
            return Err(InvalidStatusCode::new());
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    const OK: Self = StatusCode(200);
}

#[test]
fn test_valid_status_codes() {
    let ok = StatusCode::from_u16(200).unwrap();
    assert_eq!(ok, StatusCode::OK);

    let created = StatusCode::from_u16(201).unwrap();
    assert_eq!(created, StatusCode(201));

    let accepted = StatusCode::from_u16(202).unwrap();
    assert_eq!(accepted, StatusCode(202));

    let bad_request = StatusCode::from_u16(400).unwrap();
    assert_eq!(bad_request, StatusCode(400));

    let not_found = StatusCode::from_u16(404).unwrap();
    assert_eq!(not_found, StatusCode(404));

    let internal_server_error = StatusCode::from_u16(500).unwrap();
    assert_eq!(internal_server_error, StatusCode(500));
}

#[test]
fn test_invalid_status_codes_below_lower_bound() {
    let err = StatusCode::from_u16(99);
    assert!(err.is_err());
}

#[test]
fn test_invalid_status_codes_above_upper_bound() {
    let err = StatusCode::from_u16(1000);
    assert!(err.is_err());
}

#[test]
fn test_invalid_status_codes_at_lower_bound() {
    let valid = StatusCode::from_u16(100).unwrap();
    assert_eq!(valid, StatusCode(100));
}

#[test]
fn test_invalid_status_codes_at_upper_bound() {
    let valid = StatusCode::from_u16(999).unwrap();
    assert_eq!(valid, StatusCode(999));
}

