pub fn negate(&mut self) {
        if self.ranges.is_empty() {
            let (min, max) = (I::Bound::min_value(), I::Bound::max_value());
            self.ranges.push(I::create(min, max));
            return;
        }

        // There should be a way to do this in-place with constant memory,
        // but I couldn't figure out a simple way to do it. So just append
        // the negation to the end of this range, and then drain it before
        // we're done.
        let drain_end = self.ranges.len();

        // We do checked arithmetic below because of the canonical ordering
        // invariant.
        if self.ranges[0].lower() > I::Bound::min_value() {
            let upper = self.ranges[0].lower().decrement();
            self.ranges.push(I::create(I::Bound::min_value(), upper));
        }
        for i in 1..drain_end {
            let lower = self.ranges[i - 1].upper().increment();
            let upper = self.ranges[i].lower().decrement();
            self.ranges.push(I::create(lower, upper));
        }
        if self.ranges[drain_end - 1].upper() < I::Bound::max_value() {
            let lower = self.ranges[drain_end - 1].upper().increment();
            self.ranges.push(I::create(lower, I::Bound::max_value()));
        }
        self.ranges.drain(..drain_end);
    }