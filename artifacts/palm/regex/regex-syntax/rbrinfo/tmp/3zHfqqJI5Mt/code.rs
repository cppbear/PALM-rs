pub fn iter(&self) -> IntervalSetIter<I> {
        IntervalSetIter(self.ranges.iter())
    }