fn num_bytes(&self) -> usize {
        self.lits.iter().fold(0, |accum, lit| accum + lit.len())
    }