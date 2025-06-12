// Answer 0

#[test]
fn test_as_str_http() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
    }

    enum Scheme2 {
        Http,
        Https,
        Other(String),
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                Standard(Scheme2::Other(ref v)) => &v[..],
                _ => unreachable!(),
            }
        }
    }

    let scheme = Scheme {
        inner: Protocol::Standard(Scheme2::Http),
    };
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_as_str_https() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
    }

    enum Scheme2 {
        Http,
        Https,
        Other(String),
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                Standard(Scheme2::Other(ref v)) => &v[..],
                _ => unreachable!(),
            }
        }
    }

    let scheme = Scheme {
        inner: Protocol::Standard(Scheme2::Https),
    };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
fn test_as_str_other() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
    }

    enum Scheme2 {
        Http,
        Https,
        Other(String),
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                Standard(Scheme2::Other(ref v)) => &v[..],
                _ => unreachable!(),
            }
        }
    }

    let scheme = Scheme {
        inner: Protocol::Standard(Scheme2::Other("ftp".to_string())),
    };
    assert_eq!(scheme.as_str(), "ftp");
}

