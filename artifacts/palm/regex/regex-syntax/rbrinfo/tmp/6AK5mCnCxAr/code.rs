fn description(&self) -> &str {
        match *self {
            Error::Parse(ref x) => x.description(),
            Error::Translate(ref x) => x.description(),
            _ => unreachable!(),
        }
    }