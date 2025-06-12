pub fn split_first(&self) -> Option<((&K, &V), &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }