pub fn or_try_insert_with<F: FnOnce() -> T>(
        self,
        default: F,
    ) -> Result<&'a mut T, MaxSizeReached> {
        use self::Entry::*;

        match self {
            Occupied(e) => Ok(e.into_mut()),
            Vacant(e) => e.try_insert(default()),
        }
    }