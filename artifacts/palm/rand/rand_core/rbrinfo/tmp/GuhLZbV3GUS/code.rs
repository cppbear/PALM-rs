fn next_u32(&mut self) -> u32 {
        if self.index >= self.results.as_ref().len() {
            self.generate_and_set(0);
        }

        let value = self.results.as_ref()[self.index];
        self.index += 1;
        value
    }