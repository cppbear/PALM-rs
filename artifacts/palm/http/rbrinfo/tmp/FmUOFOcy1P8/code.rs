fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            u16::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }