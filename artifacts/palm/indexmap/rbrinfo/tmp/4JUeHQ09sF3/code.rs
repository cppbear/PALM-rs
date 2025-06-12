fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::key_ref);
        f.debug_list().entries(iter).finish()
    }