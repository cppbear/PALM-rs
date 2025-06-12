pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Result<U> {
        match self {
            Result::Match(t) => Result::Match(f(t)),
            Result::NoMatch(x) => Result::NoMatch(x),
            Result::Quit => Result::Quit,
        }
    }