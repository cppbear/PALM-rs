fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::with_hasher_in(Default::default(), Default::default());
        set.extend(iter);
        set
    }