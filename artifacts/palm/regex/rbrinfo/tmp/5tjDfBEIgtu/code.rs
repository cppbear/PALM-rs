fn next(&mut self) -> Option<Captures<'t>> {
        self.0.next().map(|locs| Captures {
            text: self.0.text(),
            locs: locs,
            named_groups: self.0.regex().capture_name_idx().clone(),
        })
    }