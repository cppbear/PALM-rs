// Answer 0

#[test]
fn test_fmt_http_h2() {
    use std::fmt;

    #[derive(Debug)]
    struct Http(HttpVariant);

    #[derive(Debug)]
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
    
    let http2 = Http(HttpVariant::H2);
    let result = format!("{}", http2);
    assert_eq!(result, "HTTP/2.0");
}

#[test]
fn test_fmt_http_h3() {
    use std::fmt;

    #[derive(Debug)]
    struct Http(HttpVariant);

    #[derive(Debug)]
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

    let http3 = Http(HttpVariant::H3);
    let result = format!("{}", http3);
    assert_eq!(result, "HTTP/3.0");
}

#[test]
#[should_panic]
fn test_fmt_http_non_exhaustive() {
    use std::fmt;

    #[derive(Debug)]
    struct Http(HttpVariant);

    #[derive(Debug)]
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

    let non_exhaustive = Http(HttpVariant::__NonExhaustive);
    let _ = format!("{}", non_exhaustive); // This should panic
}

