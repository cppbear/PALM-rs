pub fn split_first(&self) -> Option<(&T, &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((&first.key, Self::from_slice(rest)))
        } else {
            None
        }
    }