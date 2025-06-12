// Answer 0

#[derive(Debug)]
enum Http {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
    __NonExhaustive,
}

struct Version(Http);

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Http::*;

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

#[test]
#[should_panic]
fn test_non_exhaustive() {
    let version = Version(Http::__NonExhaustive);
    let _ = format!("{}", version);
}

