fn key_mut(&mut self) -> &mut Self::Key {
        match self {
            Entry::Occupied(e) => e.key_mut(),
            Entry::Vacant(e) => e.key_mut(),
        }
    }