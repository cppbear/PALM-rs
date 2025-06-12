// Answer 0

#[test]
fn test_fmt_valid_status_code() {
    struct Status(u16);

    impl From<Status> for u16 {
        fn from(status: Status) -> Self {
            status.0
        }
    }

    impl Status {
        fn canonical_reason(&self) -> Option<&str> {
            match self.0 {
                200 => Some("OK"),
                404 => Some("Not Found"),
                500 => Some("Internal Server Error"),
                _ => None,
            }
        }
    }

    let status_ok = Status(200);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", status_ok);
    assert!(result.is_ok());
    assert_eq!(buffer, "200 OK\n");

    buffer.clear();
    let status_not_found = Status(404);
    result = write!(&mut buffer, "{}", status_not_found);
    assert!(result.is_ok());
    assert_eq!(buffer, "404 Not Found\n");

    buffer.clear();
    let status_internal_error = Status(500);
    result = write!(&mut buffer, "{}", status_internal_error);
    assert!(result.is_ok());
    assert_eq!(buffer, "500 Internal Server Error\n");

    buffer.clear();
    let status_unknown = Status(123);
    result = write!(&mut buffer, "{}", status_unknown);
    assert!(result.is_ok());
    assert_eq!(buffer, "123 <unknown status code>\n");
}

#[test]
#[should_panic]
fn test_fmt_should_panic_on_invalid_write() {
    struct PanicStatus(u16);

    impl From<PanicStatus> for u16 {
        fn from(status: PanicStatus) -> Self {
            status.0
        }
    }

    impl PanicStatus {
        fn canonical_reason(&self) -> Option<&str> {
            panic!("Panic intended for testing");
        }
    }

    let panic_status = PanicStatus(400);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", panic_status);
}

