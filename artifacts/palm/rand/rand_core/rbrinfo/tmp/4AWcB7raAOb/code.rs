pub fn reset(&mut self) {
        self.index = self.results.as_ref().len();
        self.half_used = false;
    }