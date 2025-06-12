fn next(&mut self) -> Option<Self::Item> {
        // TODO: investigate using SliceIndex::get_unchecked when stable
        self.indices.next().map(|i| &self.slice[i])
    }