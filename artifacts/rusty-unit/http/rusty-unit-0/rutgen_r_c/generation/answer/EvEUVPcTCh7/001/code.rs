// Answer 0

#[test]
fn test_fmt_success() {
    struct HeaderValue {
        inner: Bytes,
        is_sensitive: bool,
    }

    impl fmt::Display for HeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("failed to convert header to a str")
        }
    }

    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Test Header"),
        is_sensitive: false,
    };

    let result = format!("{}", header_value);
    assert_eq!(result, "failed to convert header to a str");
}

#[test]
#[should_panic(expected = "failed to convert header to a str")]
fn test_fmt_panic() {
    struct HeaderValue {
        inner: Bytes,
        is_sensitive: bool,
    }

    impl fmt::Display for HeaderValue {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("failed to convert header to a str")
        }
    }

    let header_value = HeaderValue {
        inner: Bytes::from_static(b"Test Header"),
        is_sensitive: false,
    };

    let _ = format!("{}", header_value);
}

