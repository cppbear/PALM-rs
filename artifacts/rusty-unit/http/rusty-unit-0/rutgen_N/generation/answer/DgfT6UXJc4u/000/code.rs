// Answer 0

#[test]
fn test_http_versions() {
    use std::fmt;
    
    struct Http(HttpVersion);
    
    #[derive(Debug)]
    enum HttpVersion {
        Http09,
        Http10,
        Http11,
        H2,
        H3,
        __NonExhaustive,
    }
    
    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use self::HttpVersion::*;
        
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

    let http09 = Http(HttpVersion::Http09);
    let http10 = Http(HttpVersion::Http10);
    let http11 = Http(HttpVersion::Http11);
    let h2 = Http(HttpVersion::H2);
    let h3 = Http(HttpVersion::H3);

    assert_eq!(format!("{}", http09), "HTTP/0.9");
    assert_eq!(format!("{}", http10), "HTTP/1.0");
    assert_eq!(format!("{}", http11), "HTTP/1.1");
    assert_eq!(format!("{}", h2), "HTTP/2.0");
    assert_eq!(format!("{}", h3), "HTTP/3.0");
}

