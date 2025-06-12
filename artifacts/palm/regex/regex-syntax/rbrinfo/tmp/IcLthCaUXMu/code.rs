fn class_exceeds_limits(&self, size: usize) -> bool {
        if size > self.limit_class {
            return true;
        }
        // This is an approximation since codepoints in a char class can encode
        // to 1-4 bytes.
        let new_byte_count =
            if self.lits.is_empty() {
                size
            } else {
                self.lits
                    .iter()
                    .fold(0, |accum, lit| {
                        accum + if lit.is_cut() {
                            // If the literal is cut, then we'll never add
                            // anything to it, so don't count it.
                            0
                        } else {
                            (lit.len() + 1) * size
                        }
                    })
            };
        new_byte_count > self.limit_size
    }