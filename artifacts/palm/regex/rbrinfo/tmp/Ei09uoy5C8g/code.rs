fn from(c: Option<char>) -> Char {
        c.map_or(Char(u32::MAX), |c| c.into())
    }