fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => err.fmt(f),
            Error::CompiledTooBig(limit) => {
                write!(f, "Compiled regex exceeds size limit of {} bytes.",
                       limit)
            }
            Error::__Nonexhaustive => unreachable!(),
        }
    }