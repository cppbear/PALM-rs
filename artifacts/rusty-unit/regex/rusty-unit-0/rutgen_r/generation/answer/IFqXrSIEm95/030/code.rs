// Answer 0

#[test]
fn test_description_capture_limit_exceeded() {
    struct ErrorKind {
        kind: CaptureLimitExceeded,
    }
    
    struct CaptureLimitExceeded;

    let error = ErrorKind { kind: CaptureLimitExceeded };
    assert_eq!(error.description(), "capture group limit exceeded");
}

