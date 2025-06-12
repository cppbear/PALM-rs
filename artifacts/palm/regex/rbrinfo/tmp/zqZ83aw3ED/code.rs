fn next(&mut self) -> Option<Locations> {
        if self.0.last_end > self.0.text.as_ref().len() {
            return None
        }
        let mut locs = self.0.re.locations();
        let (s, e) = match self.0.re.read_captures_at(
            &mut locs,
            self.0.text,
            self.0.last_end,
        ) {
            None => return None,
            Some((s, e)) => (s, e),
        };
        if s == e {
            self.0.last_end = self.0.re.next_after_empty(self.0.text, e);
            if Some(e) == self.0.last_match {
                return self.next();
            }
        } else {
            self.0.last_end = e;
        }
        self.0.last_match = Some(e);
        Some(locs)
    }