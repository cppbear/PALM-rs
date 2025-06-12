fn set_non_match(self, at: usize) -> Result<T> {
        match self {
            Result::NoMatch(_) => Result::NoMatch(at),
            r => r,
        }
    }