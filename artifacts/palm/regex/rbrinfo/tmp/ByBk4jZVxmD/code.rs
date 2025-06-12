fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => {
                let hr: String = repeat('~').take(79).collect();
                writeln!(f, "Syntax(")?;
                writeln!(f, "{}", hr)?;
                writeln!(f, "{}", err)?;
                writeln!(f, "{}", hr)?;
                write!(f, ")")?;
                Ok(())
            }
            Error::CompiledTooBig(limit) => {
                f.debug_tuple("CompiledTooBig")
                    .field(&limit)
                    .finish()
            }
            Error::__Nonexhaustive => {
                f.debug_tuple("__Nonexhaustive").finish()
            }
        }
    }