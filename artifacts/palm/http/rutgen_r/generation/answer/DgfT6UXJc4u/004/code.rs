// Answer 0

#[test]
fn test_fmt_http_11() {
    use std::fmt;

    struct Http(HttpKind);
    
    enum HttpKind {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpKind::*;

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

    let http11 = Http(HttpKind::Http11);
    let mut output = String::new();
    let result = http11.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.1");
}

