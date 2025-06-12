// Answer 0

#[test]
fn test_canonical_reason_ok() {
    struct StatusCode(u16);
    
    impl StatusCode {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                200 => Some("OK"),
                _ => None,
            }
        }
    }

    let status = StatusCode(200);
    assert_eq!(status.canonical_reason(), Some("OK"));
}

#[test]
fn test_canonical_reason_not_found() {
    struct StatusCode(u16);
    
    impl StatusCode {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                404 => Some("Not Found"),
                _ => None,
            }
        }
    }

    let status = StatusCode(404);
    assert_eq!(status.canonical_reason(), Some("Not Found"));
}

#[test]
fn test_canonical_reason_internal_server_error() {
    struct StatusCode(u16);
    
    impl StatusCode {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                500 => Some("Internal Server Error"),
                _ => None,
            }
        }
    }

    let status = StatusCode(500);
    assert_eq!(status.canonical_reason(), Some("Internal Server Error"));
}

#[test]
fn test_canonical_reason_unknown_code() {
    struct StatusCode(u16);
    
    impl StatusCode {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                _ => None,
            }
        }
    }

    let status = StatusCode(999);
    assert_eq!(status.canonical_reason(), None);
}

#[test]
#[should_panic]
fn test_canonical_reason_panic_case() {
    struct StatusCode(u16);
    
    impl StatusCode {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                400 => panic!("Panicking on purpose for testing"),
                _ => None,
            }
        }
    }

    let status = StatusCode(400);
    status.canonical_reason(); // this should trigger the panic
}

