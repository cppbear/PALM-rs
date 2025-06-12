fn skip_to_escape_slow(&mut self) {
        while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
            self.index += 1;
        }
    }