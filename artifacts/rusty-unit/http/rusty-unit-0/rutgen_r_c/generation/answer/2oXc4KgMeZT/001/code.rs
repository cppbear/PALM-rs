// Answer 0

#[test]
fn test_as_str_http() {
    struct Protocol;
    impl Protocol {
        const Http: Self = Protocol;
    }

    let scheme = super::Scheme {
        inner: super::Scheme2::Standard(Protocol::Http),
    };
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_as_str_https() {
    struct Protocol;
    impl Protocol {
        const Https: Self = Protocol;
    }

    let scheme = super::Scheme {
        inner: super::Scheme2::Standard(Protocol::Https),
    };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
fn test_as_str_other() {
    use bytes::Bytes;

    struct Protocol;
    impl Protocol {
        const Http: Self = Protocol;
        const Https: Self = Protocol;
    }

    let bytes = Bytes::from("custom_scheme");
    let scheme = super::Scheme {
        inner: super::Scheme2::Other(Box::new(super::ByteStr { bytes })),
    };
    assert_eq!(scheme.as_str(), "custom_scheme");
}

#[should_panic(expected = "panicked at 'called `Option::unwrap()` on a `None` value'")]
#[test]
fn test_as_str_unreachable() {
    let scheme = super::Scheme {
        inner: super::Scheme2::None,
    };
    let _ = scheme.as_str(); // This should panic, as expected
}

