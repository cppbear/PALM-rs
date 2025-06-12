fn clone(&self) -> Self {
        WeightedIndexIter {
            weighted_index: self.weighted_index,
            index: self.index,
        }
    }