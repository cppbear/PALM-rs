// Answer 0

#[test]
fn test_scheme_as_str_http() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
        Other(String),
        None,
    }

    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Http) => "http",
                Standard(Https) => "https",
                Other(ref v) => &v[..],
                None => unreachable!(),
            }
        }
    }

    let scheme: Scheme = Scheme { inner: Protocol::Standard(Scheme2::Http) };
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_scheme_as_str_https() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
        Other(String),
        None,
    }

    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Http) => "http",
                Standard(Https) => "https",
                Other(ref v) => &v[..],
                None => unreachable!(),
            }
        }
    }

    let scheme: Scheme = Scheme { inner: Protocol::Standard(Scheme2::Https) };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
fn test_scheme_as_str_other() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
        Other(String),
        None,
    }

    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Http) => "http",
                Standard(Https) => "https",
                Other(ref v) => &v[..],
                None => unreachable!(),
            }
        }
    }

    let scheme: Scheme = Scheme { inner: Protocol::Other("ftp".to_string()) };
    assert_eq!(scheme.as_str(), "ftp");
}

