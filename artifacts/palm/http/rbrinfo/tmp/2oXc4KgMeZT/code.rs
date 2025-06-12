pub fn as_str(&self) -> &str {
        use self::Protocol::*;
        use self::Scheme2::*;

        match self.inner {
            Standard(Http) => "http",
            Standard(Https) => "https",
            Other(ref v) => &v[..],
            None => unreachable!(),
        }
    }