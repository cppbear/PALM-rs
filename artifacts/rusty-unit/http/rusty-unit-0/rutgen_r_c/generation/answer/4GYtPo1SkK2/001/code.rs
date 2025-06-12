// Answer 0

#[test]
fn test_scheme_is_none_standard_http() {
    struct StandardHttp;

    impl FromStr for StandardHttp {
        type Err = std::convert::Infallible;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            Ok(StandardHttp)
        }
    }

    let scheme = Scheme2::Standard(Protocol::Http);
    assert_eq!(scheme.is_none(), false);
}

#[test]
fn test_scheme_is_none_standard_https() {
    let scheme = Scheme2::Standard(Protocol::Https);
    assert_eq!(scheme.is_none(), false);
}

#[test]
fn test_scheme_is_none_other() {
    struct OtherData;

    let scheme = Scheme2::Other(Box::new(ByteStr::from_static(b"custom_scheme")));
    assert_eq!(scheme.is_none(), false);
}

