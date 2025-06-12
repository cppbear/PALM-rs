// Answer 0

#[test]
fn test_canonical_reason_for_successful_status_codes() {
    struct DummyStatusCode(NonZeroU16);
    impl StatusCode {
        const fn new(num: u16) -> Self {
            StatusCode(unsafe { NonZeroU16::new_unchecked(num) })
        }
    }

    let status_200 = DummyStatusCode::new(200);
    assert_eq!(status_200.canonical_reason(), Some("OK"));

    let status_201 = DummyStatusCode::new(201);
    assert_eq!(status_201.canonical_reason(), Some("Created"));

    let status_202 = DummyStatusCode::new(202);
    assert_eq!(status_202.canonical_reason(), Some("Accepted"));
}

#[test]
fn test_canonical_reason_for_client_error_status_codes() {
    struct DummyStatusCode(NonZeroU16);
    impl StatusCode {
        const fn new(num: u16) -> Self {
            StatusCode(unsafe { NonZeroU16::new_unchecked(num) })
        }
    }

    let status_404 = DummyStatusCode::new(404);
    assert_eq!(status_404.canonical_reason(), Some("Not Found"));

    let status_403 = DummyStatusCode::new(403);
    assert_eq!(status_403.canonical_reason(), Some("Forbidden"));
}

#[test]
fn test_canonical_reason_for_server_error_status_codes() {
    struct DummyStatusCode(NonZeroU16);
    impl StatusCode {
        const fn new(num: u16) -> Self {
            StatusCode(unsafe { NonZeroU16::new_unchecked(num) })
        }
    }

    let status_500 = DummyStatusCode::new(500);
    assert_eq!(status_500.canonical_reason(), Some("Internal Server Error"));

    let status_502 = DummyStatusCode::new(502);
    assert_eq!(status_502.canonical_reason(), Some("Bad Gateway"));
}

#[test]
fn test_canonical_reason_for_unknown_status_code() {
    struct DummyStatusCode(NonZeroU16);
    impl StatusCode {
        const fn new(num: u16) -> Self {
            StatusCode(unsafe { NonZeroU16::new_unchecked(num) })
        }
    }

    let status_999 = DummyStatusCode::new(999);
    assert_eq!(status_999.canonical_reason(), None);
}

