fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);
    }