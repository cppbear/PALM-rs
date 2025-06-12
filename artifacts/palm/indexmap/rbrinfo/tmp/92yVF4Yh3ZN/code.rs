pub fn split_last(&self) -> Option<(&T, &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((&last.key, Self::from_slice(rest)))
        } else {
            None
        }
    }