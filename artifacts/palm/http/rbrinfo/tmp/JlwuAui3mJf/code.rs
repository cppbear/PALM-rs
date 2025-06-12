pub fn key(&self) -> &HeaderName {
        use self::Entry::*;

        match *self {
            Vacant(ref e) => e.key(),
            Occupied(ref e) => e.key(),
        }
    }