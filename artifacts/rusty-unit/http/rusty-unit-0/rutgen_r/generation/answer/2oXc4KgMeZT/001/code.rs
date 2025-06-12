// Answer 0

#[derive(Debug)]
struct Scheme {
    inner: InnerScheme,
}

#[derive(Debug)]
enum InnerScheme {
    Standard(Protocol),
    Other(String),
    None,
}

#[derive(Debug)]
enum Protocol {
    Http,
    Https,
}

impl Scheme {
    pub fn as_str(&self) -> &str {
        use Protocol::*;
        use InnerScheme::*;

        match &self.inner {
            Standard(Http) => "http",
            Standard(Https) => "https",
            Other(ref v) => &v[..],
            None => unreachable!(),
        }
    }
}

#[test]
fn test_as_str_standard_http() {
    let scheme = Scheme {
        inner: InnerScheme::Standard(Protocol::Http),
    };
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_as_str_standard_https() {
    let scheme = Scheme {
        inner: InnerScheme::Standard(Protocol::Https),
    };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
fn test_as_str_other() {
    let scheme = Scheme {
        inner: InnerScheme::Other("ftp".to_string()),
    };
    assert_eq!(scheme.as_str(), "ftp");
}

#[should_panic]
#[test]
fn test_as_str_other_empty_string() {
    let scheme = Scheme {
        inner: InnerScheme::Other("".to_string()),
    };
    // This should panic as v[..] is attempting to slice an empty string
    let _ = scheme.as_str();
}

