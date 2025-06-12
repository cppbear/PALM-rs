use std::fmt;
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Http {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
    __NonExhaustive,
}
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
