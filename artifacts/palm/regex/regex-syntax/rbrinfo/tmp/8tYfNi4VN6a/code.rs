fn canonicalize(&mut self) {
        if self.is_canonical() {
            return;
        }
        self.ranges.sort();
        assert!(!self.ranges.is_empty());

        // Is there a way to do this in-place with constant memory? I couldn't
        // figure out a way to do it. So just append the canonicalization to
        // the end of this range, and then drain it before we're done.
        let drain_end = self.ranges.len();
        for oldi in 0..drain_end {
            // If we've added at least one new range, then check if we can
            // merge this range in the previously added range.
            if self.ranges.len() > drain_end {
                let (last, rest) = self.ranges.split_last_mut().unwrap();
                if let Some(union) = last.union(&rest[oldi]) {
                    *last = union;
                    continue;
                }
            }
            let range = self.ranges[oldi];
            self.ranges.push(range);
        }
        self.ranges.drain(..drain_end);
    }