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

    #[derive(Debug)]
    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                _ => unreachable!(),
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

    #[derive(Debug)]
    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                _ => unreachable!(),
            }
        }
    }

    let scheme: Scheme = Scheme { inner: Protocol::Standard(Scheme2::Https) };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
#[should_panic(expected = "unreachable!()")]
fn test_scheme_as_str_none() {
    struct Scheme {
        inner: Protocol,
    }

    enum Protocol {
        Standard(Scheme2),
        Other(String),
        None,
    }

    #[derive(Debug)]
    enum Scheme2 {
        Http,
        Https,
    }

    impl Scheme {
        pub fn as_str(&self) -> &str {
            use self::Protocol::*;
            use self::Scheme2::*;

            match self.inner {
                Standard(Scheme2::Http) => "http",
                Standard(Scheme2::Https) => "https",
                _ => unreachable!(),
            }
        }
    }

    let scheme: Scheme = Scheme { inner: Protocol::None };
    scheme.as_str(); // This will panic
}

