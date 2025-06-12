// Answer 0

#[test]
fn test_as_str_valid_port() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let port = Port::from_str(TestString("80")).unwrap();
    assert_eq!(port.as_str(), "80");
}

#[test]
fn test_as_str_multiple_digits_port() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let port = Port::from_str(TestString("65535")).unwrap();
    assert_eq!(port.as_str(), "65535");
}

#[test]
fn test_as_str_edge_case_zero_port() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let port = Port::from_str(TestString("0")).unwrap();
    assert_eq!(port.as_str(), "0");
}

#[should_panic]
fn test_as_str_invalid_port() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let _port = Port::from_str(TestString("invalid")).unwrap();
}

#[test]
fn test_as_str_minimum_valid_port() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let port = Port::from_str(TestString("1")).unwrap();
    assert_eq!(port.as_str(), "1");
}

#[test]
fn test_as_str_boundaries() {
    struct TestString<'a>(&'a str);
    impl AsRef<str> for TestString<'_> {
        fn as_ref(&self) -> &str {
            self.0
        }
    }

    let port = Port::from_str(TestString("65535")).unwrap();
    assert_eq!(port.as_str(), "65535");

    let port_zero = Port::from_str(TestString("0")).unwrap();
    assert_eq!(port_zero.as_str(), "0");
}

