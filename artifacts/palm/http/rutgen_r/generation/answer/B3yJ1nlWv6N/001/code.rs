// Answer 0

#[test]
fn test_fmt_response() {
    struct Response {
        status: &'static str,
        version: &'static str,
        headers: &'static str,
        body: &'static str,
    }

    impl Response {
        fn status(&self) -> &str {
            self.status
        }

        fn version(&self) -> &str {
            self.version
        }

        fn headers(&self) -> &str {
            self.headers
        }

        fn body(&self) -> &str {
            self.body
        }
    }

    let response = Response {
        status: "200 OK",
        version: "HTTP/1.1",
        headers: "Content-Type: text/html",
        body: "<html>Hello, world!</html>",
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", response));

    assert!(result.is_ok());
    assert!(output.contains("Response"));
    assert!(output.contains("status: \"200 OK\""));
    assert!(output.contains("version: \"HTTP/1.1\""));
    assert!(output.contains("headers: \"Content-Type: text/html\""));
    assert!(output.contains("body: \"<html>Hello, world!</html>\""));
}

#[test]
fn test_fmt_empty_response() {
    struct Response {
        status: &'static str,
        version: &'static str,
        headers: &'static str,
        body: &'static str,
    }

    impl Response {
        fn status(&self) -> &str {
            self.status
        }

        fn version(&self) -> &str {
            self.version
        }

        fn headers(&self) -> &str {
            self.headers
        }

        fn body(&self) -> &str {
            self.body
        }
    }

    let response = Response {
        status: "",
        version: "",
        headers: "",
        body: "",
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", response));

    assert!(result.is_ok());
    assert!(output.contains("Response"));
    assert!(output.contains("status: \"\""));
    assert!(output.contains("version: \"\""));
    assert!(output.contains("headers: \"\""));
    assert!(output.contains("body: \"\""));
}

#[should_panic]
#[test]
fn test_fmt_response_with_panic() {
    struct Response {
        status: &'static str,
        version: &'static str,
        headers: &'static str,
        body: &'static str,
    }

    impl Response {
        fn status(&self) -> &str {
            self.status
        }

        fn version(&self) -> &str {
            self.version
        }

        fn headers(&self) -> &str {
            self.headers
        }

        fn body(&self) -> &str {
            self.body
        }
    }

    let response = Response {
        status: "500 Internal Server Error",
        version: "HTTP/1.0",
        headers: "Content-Length: 0",
        body: "",
    };

    // Intentional panic scenario
    panic!("Simulating a panic in formatting");
}

