fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Tried to create a `rand::distr::slice::Choose` with an empty slice"
        )
    }