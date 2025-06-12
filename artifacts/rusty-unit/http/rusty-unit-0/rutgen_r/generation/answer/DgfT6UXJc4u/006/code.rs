// Answer 0

#[test]
fn test_fmt_http09() {
    use std::fmt;

    struct Http(HttpVariant);
    enum HttpVariant {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVariant::*;

            f.write_str(match self.0 {
                Http09 => "HTTP/0.9",
                Http10 => "HTTP/1.0",
                Http11 => "HTTP/1.1",
                H2 => "HTTP/2.0",
                H3 => "HTTP/3.0",
                __NonExhaustive => unreachable!(),
            })
        }
    }

    let http = Http(HttpVariant::Http09);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/0.9");
}

#[test]
fn test_fmt_http10() {
    use std::fmt;

    struct Http(HttpVariant);
    enum HttpVariant {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVariant::*;

            f.write_str(match self.0 {
                Http09 => "HTTP/0.9",
                Http10 => "HTTP/1.0",
                Http11 => "HTTP/1.1",
                H2 => "HTTP/2.0",
                H3 => "HTTP/3.0",
                __NonExhaustive => unreachable!(),
            })
        }
    }

    let http = Http(HttpVariant::Http10);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.0");
}

#[test]
fn test_fmt_http11() {
    use std::fmt;

    struct Http(HttpVariant);
    enum HttpVariant {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVariant::*;

            f.write_str(match self.0 {
                Http09 => "HTTP/0.9",
                Http10 => "HTTP/1.0",
                Http11 => "HTTP/1.1",
                H2 => "HTTP/2.0",
                H3 => "HTTP/3.0",
                __NonExhaustive => unreachable!(),
            })
        }
    }

    let http = Http(HttpVariant::Http11);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.1");
}

#[test]
fn test_fmt_h2() {
    use std::fmt;

    struct Http(HttpVariant);
    enum HttpVariant {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVariant::*;

            f.write_str(match self.0 {
                Http09 => "HTTP/0.9",
                Http10 => "HTTP/1.0",
                Http11 => "HTTP/1.1",
                H2 => "HTTP/2.0",
                H3 => "HTTP/3.0",
                __NonExhaustive => unreachable!(),
            })
        }
    }

    let http = Http(HttpVariant::H2);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/2.0");
}

#[test]
fn test_fmt_h3() {
    use std::fmt;

    struct Http(HttpVariant);
    enum HttpVariant {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVariant::*;

            f.write_str(match self.0 {
                Http09 => "HTTP/0.9",
                Http10 => "HTTP/1.0",
                Http11 => "HTTP/1.1",
                H2 => "HTTP/2.0",
                H3 => "HTTP/3.0",
                __NonExhaustive => unreachable!(),
            })
        }
    }

    let http = Http(HttpVariant::H3);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/3.0");
}

