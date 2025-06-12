pub fn is_all_ascii(&self) -> bool {
        self.set.intervals().last().map_or(true, |r| r.end <= 0x7F)
    }