pub fn is_match(&self) -> bool {
        match *self {
            Inst::Match(_) => true,
            _ => false,
        }
    }