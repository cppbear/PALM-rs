pub fn difference(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }

        // This algorithm is (to me) surprisingly complex. A search of the
        // interwebs indicate that this is a potentially interesting problem.
        // Folks seem to suggest interval or segment trees, but I'd like to
        // avoid the overhead (both runtime and conceptual) of that.
        //
        // The following is basically my Shitty First Draft. Therefore, in
        // order to grok it, you probably need to read each line carefully.
        // Simplifications are most welcome!
        //
        // Remember, we can assume the canonical format invariant here, which
        // says that all ranges are sorted, not overlapping and not adjacent in
        // each class.
        let drain_end = self.ranges.len();
        let (mut a, mut b) = (0, 0);
    'LOOP:
        while a < drain_end && b < other.ranges.len() {
            // Basically, the easy cases are when neither range overlaps with
            // each other. If the `b` range is less than our current `a`
            // range, then we can skip it and move on.
            if other.ranges[b].upper() < self.ranges[a].lower() {
                b += 1;
                continue;
            }
            // ... similarly for the `a` range. If it's less than the smallest
            // `b` range, then we can add it as-is.
            if self.ranges[a].upper() < other.ranges[b].lower() {
                let range = self.ranges[a];
                self.ranges.push(range);
                a += 1;
                continue;
            }
            // Otherwise, we have overlapping ranges.
            assert!(!self.ranges[a].is_intersection_empty(&other.ranges[b]));

            // This part is tricky and was non-obvious to me without looking
            // at explicit examples (see the tests). The trickiness stems from
            // two things: 1) subtracting a range from another range could
            // yield two ranges and 2) after subtracting a range, it's possible
            // that future ranges can have an impact. The loop below advances
            // the `b` ranges until they can't possible impact the current
            // range.
            //
            // For example, if our `a` range is `a-t` and our next three `b`
            // ranges are `a-c`, `g-i`, `r-t` and `x-z`, then we need to apply
            // subtraction three times before moving on to the next `a` range.
            let mut range = self.ranges[a];
            while b < other.ranges.len()
                && !range.is_intersection_empty(&other.ranges[b])
            {
                let old_range = range;
                range = match range.difference(&other.ranges[b]) {
                    (None, None) => {
                        // We lost the entire range, so move on to the next
                        // without adding this one.
                        a += 1;
                        continue 'LOOP;
                    }
                    (Some(range1), None) | (None, Some(range1)) => range1,
                    (Some(range1), Some(range2)) => {
                        self.ranges.push(range1);
                        range2
                    }
                };
                // It's possible that the `b` range has more to contribute
                // here. In particular, if it is greater than the original
                // range, then it might impact the next `a` range *and* it
                // has impacted the current `a` range as much as possible,
                // so we can quit. We don't bump `b` so that the next `a`
                // range can apply it.
                if other.ranges[b].upper() > old_range.upper() {
                    break;
                }
                // Otherwise, the next `b` range might apply to the current
                // `a` range.
                b += 1;
            }
            self.ranges.push(range);
            a += 1;
        }
        while a < drain_end {
            let range = self.ranges[a];
            self.ranges.push(range);
            a += 1;
        }
        self.ranges.drain(..drain_end);
    }