pub fn intersect(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() {
            return;
        }
        if other.ranges.is_empty() {
            self.ranges.clear();
            return;
        }

        // There should be a way to do this in-place with constant memory,
        // but I couldn't figure out a simple way to do it. So just append
        // the intersection to the end of this range, and then drain it before
        // we're done.
        let drain_end = self.ranges.len();

        let mut ita = (0..drain_end).into_iter();
        let mut itb = (0..other.ranges.len()).into_iter();
        let mut a = ita.next().unwrap();
        let mut b = itb.next().unwrap();
        loop {
            if let Some(ab) = self.ranges[a].intersect(&other.ranges[b]) {
                self.ranges.push(ab);
            }
            let (it, aorb) =
                if self.ranges[a].upper() < other.ranges[b].upper() {
                    (&mut ita, &mut a)
                } else {
                    (&mut itb, &mut b)
                };
            match it.next() {
                Some(v) => *aorb = v,
                None => break,
            }
        }
        self.ranges.drain(..drain_end);
    }