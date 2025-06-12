fn next_u32(&mut self) -> u32 {
        let mut index = self.index - self.half_used as usize;
        if index >= self.results.as_ref().len() {
            self.core.generate(&mut self.results);
            self.index = 0;
            index = 0;
            // `self.half_used` is by definition `false`
            self.half_used = false;
        }

        let shift = 32 * (self.half_used as usize);

        self.half_used = !self.half_used;
        self.index += self.half_used as usize;

        (self.results.as_ref()[index] >> shift) as u32
    }