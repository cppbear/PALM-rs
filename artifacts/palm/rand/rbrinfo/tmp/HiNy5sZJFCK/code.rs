fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::EmptyRange => "low > high (or equal if exclusive) in uniform distribution",
            Error::NonFinite => "Non-finite range in uniform distribution",
        })
    }