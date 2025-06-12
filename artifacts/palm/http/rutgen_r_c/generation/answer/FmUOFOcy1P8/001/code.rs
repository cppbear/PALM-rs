// Answer 0

#[test]
fn test_fmt_status_code_success() {
    struct TestStatusCode(pub StatusCode);

    impl fmt::Display for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", u16::from(self.0))
        }
    }

    let status = TestStatusCode(StatusCode(unsafe { NonZeroU16::new_unchecked(200) }));
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);

    assert!(result.is_ok());
    assert_eq!(output, "200 OK");
}

#[test]
fn test_fmt_status_code_not_found() {
    struct TestStatusCode(pub StatusCode);

    impl fmt::Display for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", u16::from(self.0))
        }
    }

    let status = TestStatusCode(StatusCode(unsafe { NonZeroU16::new_unchecked(404) }));
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);

    assert!(result.is_ok());
    assert_eq!(output, "404 Not Found");
}

#[test]
fn test_fmt_status_code_internal_server_error() {
    struct TestStatusCode(pub StatusCode);

    impl fmt::Display for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", u16::from(self.0))
        }
    }

    let status = TestStatusCode(StatusCode(unsafe { NonZeroU16::new_unchecked(500) }));
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);

    assert!(result.is_ok());
    assert_eq!(output, "500 Internal Server Error");
}

#[test]
fn test_fmt_status_code_edge_case() {
    struct TestStatusCode(pub StatusCode);

    impl fmt::Display for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", u16::from(self.0))
        }
    }

    let status = TestStatusCode(StatusCode(unsafe { NonZeroU16::new_unchecked(418) })); // I'm a teapot
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);

    assert!(result.is_ok());
    assert_eq!(output, "418 I'm a teapot");
}

#[test]
fn test_fmt_status_code_unknown() {
    struct TestStatusCode(pub StatusCode);

    impl fmt::Display for TestStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", u16::from(self.0))
        }
    }

    let status = TestStatusCode(StatusCode(unsafe { NonZeroU16::new_unchecked(999) })); // Unknown status
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);

    assert!(result.is_ok());
    assert_eq!(output, "999 <unknown status code>");
}

