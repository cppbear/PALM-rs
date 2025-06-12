// Answer 0

#[test]
fn test_status_code_as_u16_ok() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub const OK: StatusCode = StatusCode(200);

        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }

    let status = StatusCode::OK;
    assert_eq!(status.as_u16(), 200);
}

#[test]
fn test_status_code_as_u16_not_found() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub const NOT_FOUND: StatusCode = StatusCode(404);

        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }

    let status = StatusCode::NOT_FOUND;
    assert_eq!(status.as_u16(), 404);
}

#[test]
fn test_status_code_as_u16_internal_server_error() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);

        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }

    let status = StatusCode::INTERNAL_SERVER_ERROR;
    assert_eq!(status.as_u16(), 500);
}

