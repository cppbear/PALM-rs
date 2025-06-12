pub fn new<T: IntoIterator<Item=I>>(intervals: T) -> IntervalSet<I> {
        let mut set = IntervalSet { ranges: intervals.into_iter().collect() };
        set.canonicalize();
        set
    }