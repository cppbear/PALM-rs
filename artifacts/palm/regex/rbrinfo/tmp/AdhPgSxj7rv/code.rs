pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        let mut locs = self.locations();
        self.read_captures_at(&mut locs, text, 0).map(|_| Captures {
            text: text,
            locs: locs,
            named_groups: self.0.capture_name_idx().clone(),
        })
    }