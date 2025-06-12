pub fn is_valid(&self) -> bool {
        match *self {
            RepetitionRange::Bounded(s, e) if s > e => false,
            _ => true,
        }
    }