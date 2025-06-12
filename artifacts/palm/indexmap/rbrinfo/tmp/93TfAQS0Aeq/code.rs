pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }