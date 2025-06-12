fn description(&self) -> &str {
        match *self {
            Error::Syntax(ref err) => err,
            Error::CompiledTooBig(_) => "compiled program too big",
            Error::__Nonexhaustive => unreachable!(),
        }
    }