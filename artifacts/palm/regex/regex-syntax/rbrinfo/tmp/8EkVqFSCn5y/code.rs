fn next(&mut self) -> Option<&'static [(char, char)]> {
        if self.ages.is_empty() {
            None
        } else {
            let set = self.ages[0];
            self.ages = &self.ages[1..];
            Some(set.1)
        }
    }