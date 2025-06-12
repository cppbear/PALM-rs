pub fn or_try_insert(self, default: T) -> Result<&'a mut T, MaxSizeReached> {
        use self::Entry::*;

        match self {
            Occupied(e) => Ok(e.into_mut()),
            Vacant(e) => e.try_insert(default),
        }
    }