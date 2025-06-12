pub fn is_match(&self) -> bool {
        match *self {
            Result::Match(_) => true,
            Result::NoMatch(_) | Result::Quit => false,
        }
    }