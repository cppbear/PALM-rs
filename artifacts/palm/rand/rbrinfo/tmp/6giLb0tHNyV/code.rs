fn next(&mut self) -> Option<Self::Item> {
        match self.weighted_index.weight(self.index) {
            None => None,
            Some(weight) => {
                self.index += 1;
                Some(weight)
            }
        }
    }