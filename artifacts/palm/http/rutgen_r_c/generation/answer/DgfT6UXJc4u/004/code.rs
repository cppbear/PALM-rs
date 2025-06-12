// Answer 0

#[test]
fn test_version_http11_debug() {
    use std::fmt;
    
    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    enum Http {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }
    
    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    pub struct Version(Http);
    
    impl fmt::Debug for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::Http::*;
            f.write_str(
                match self.0 {
                    Http09 => "HTTP/0.9",
                    Http10 => "HTTP/1.0",
                    Http11 => "HTTP/1.1",
                    H2 => "HTTP/2.0",
                    H3 => "HTTP/3.0",
                    __NonExhaustive => unreachable!(),
                },
            )
        }
    }
    
    let version = Version(Http::Http11);
    let mut output = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "HTTP/1.1");
}

#[test]
fn test_version_http09_debug() {
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    enum Http {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    pub struct Version(Http);

    impl fmt::Debug for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::Http::*;
            f.write_str(
                match self.0 {
                    Http09 => "HTTP/0.9",
                    Http10 => "HTTP/1.0",
                    Http11 => "HTTP/1.1",
                    H2 => "HTTP/2.0",
                    H3 => "HTTP/3.0",
                    __NonExhaustive => unreachable!(),
                },
            )
        }
    }

    let version = Version(Http::Http09);
    let mut output = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "HTTP/0.9");
}

#[test]
fn test_version_http10_debug() {
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    enum Http {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    pub struct Version(Http);

    impl fmt::Debug for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::Http::*;
            f.write_str(
                match self.0 {
                    Http09 => "HTTP/0.9",
                    Http10 => "HTTP/1.0",
                    Http11 => "HTTP/1.1",
                    H2 => "HTTP/2.0",
                    H3 => "HTTP/3.0",
                    __NonExhaustive => unreachable!(),
                },
            )
        }
    }

    let version = Version(Http::Http10);
    let mut output = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "HTTP/1.0");
}

#[test]
fn test_version_http2_debug() {
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    enum Http {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    pub struct Version(Http);

    impl fmt::Debug for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::Http::*;
            f.write_str(
                match self.0 {
                    Http09 => "HTTP/0.9",
                    Http10 => "HTTP/1.0",
                    Http11 => "HTTP/1.1",
                    H2 => "HTTP/2.0",
                    H3 => "HTTP/3.0",
                    __NonExhaustive => unreachable!(),
                },
            )
        }
    }

    let version = Version(Http::H2);
    let mut output = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "HTTP/2.0");
}

#[test]
fn test_version_http3_debug() {
    use std::fmt;

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    enum Http {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
    pub struct Version(Http);

    impl fmt::Debug for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::Http::*;
            f.write_str(
                match self.0 {
                    Http09 => "HTTP/0.9",
                    Http10 => "HTTP/1.0",
                    Http11 => "HTTP/1.1",
                    H2 => "HTTP/2.0",
                    H3 => "HTTP/3.0",
                    __NonExhaustive => unreachable!(),
                },
            )
        }
    }

    let version = Version(Http::H3);
    let mut output = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "HTTP/3.0");
}

