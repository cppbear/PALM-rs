fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeightedIndexIter")
            .field("weighted_index", &self.weighted_index)
            .field("index", &self.index)
            .finish()
    }