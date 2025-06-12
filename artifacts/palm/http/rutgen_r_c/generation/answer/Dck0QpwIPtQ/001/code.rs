// Answer 0

#[test]
fn test_canonical_reason_valid_status_codes() {
    struct TestStatusCode {
        code: StatusCode,
        expected_reason: Option<&'static str>,
    }

    let test_cases = [
        TestStatusCode { code: StatusCode::OK, expected_reason: Some("OK") },
        TestStatusCode { code: StatusCode::CREATED, expected_reason: Some("Created") },
        TestStatusCode { code: StatusCode::ACCEPTED, expected_reason: Some("Accepted") },
        TestStatusCode { code: StatusCode::NON_AUTHORITATIVE_INFORMATION, expected_reason: Some("Non Authoritative Information") },
        TestStatusCode { code: StatusCode::NOT_FOUND, expected_reason: Some("Not Found") },
        TestStatusCode { code: StatusCode::INTERNAL_SERVER_ERROR, expected_reason: Some("Internal Server Error") },
        TestStatusCode { code: StatusCode::IM_A_TEAPOT, expected_reason: Some("I'm a teapot") },
        TestStatusCode { code: StatusCode::NETWORK_AUTHENTICATION_REQUIRED, expected_reason: Some("Network Authentication Required") },
    ];

    for case in test_cases.iter() {
        assert_eq!(case.code.canonical_reason(), case.expected_reason);
    }
}

#[test]
fn test_canonical_reason_invalid_status_code() {
    struct InvalidStatusCode {
        code: StatusCode,
    }

    let invalid_cases = [
        InvalidStatusCode { code: StatusCode(NonZeroU16::new_unchecked(999)) }, // Status code out of range
        InvalidStatusCode { code: StatusCode(NonZeroU16::new_unchecked(0)) },   // Invalid status code
    ];

    for case in invalid_cases.iter() {
        assert_eq!(case.code.canonical_reason(), None);
    }
}

