fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parse(ref x) => x.fmt(f),
            Error::Translate(ref x) => x.fmt(f),
            _ => unreachable!(),
        }
    }