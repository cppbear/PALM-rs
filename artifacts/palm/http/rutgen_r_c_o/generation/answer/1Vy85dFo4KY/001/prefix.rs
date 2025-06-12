// Answer 0

#[test]
fn test_from_name_valid() {
    struct TestHeaderName {
        inner: String,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(_: TestHeaderName) -> HeaderValue {
            HeaderValue::from_static("accept")
        }
    }

    let header_name = TestHeaderName { inner: String::from("accept") };
    let val = HeaderValue::from_name(header_name);
}

#[test]
fn test_from_name_empty() {
    struct TestHeaderName {
        inner: String,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(_: TestHeaderName) -> HeaderValue {
            HeaderValue::from_static("")
        }
    }

    let header_name = TestHeaderName { inner: String::from("") };
    let val = HeaderValue::from_name(header_name);
}

#[test]
#[should_panic]
fn test_from_name_invalid() {
    struct TestHeaderName {
        inner: String,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(_: TestHeaderName) -> HeaderValue {
            panic!("Invalid header name");
        }
    }

    let header_name = TestHeaderName { inner: String::from("invalid#name") };
    let val = HeaderValue::from_name(header_name);
}

#[test]
fn test_from_name_case_insensitive() {
    struct TestHeaderName {
        inner: String,
    }

    impl From<TestHeaderName> for HeaderValue {
        fn from(_: TestHeaderName) -> HeaderValue {
            HeaderValue::from_static("ACCEPT")
        }
    }

    let header_name = TestHeaderName { inner: String::from("ACCEPT") };
    let val = HeaderValue::from_name(header_name);
}

#[test]
fn test_from_name_large_name() {
    struct TestHeaderName {
        inner: String,
    }

    let long_name = "a".repeat(20);
    
    impl From<TestHeaderName> for HeaderValue {
        fn from(_: TestHeaderName) -> HeaderValue {
            HeaderValue::from_static(long_name.as_str())
        }
    }

    let header_name = TestHeaderName { inner: long_name };
    let val = HeaderValue::from_name(header_name);
}

