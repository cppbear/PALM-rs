pub fn new<I>(ranges: I) -> ClassBytes
    where I: IntoIterator<Item=ClassBytesRange>
    {
        ClassBytes { set: IntervalSet::new(ranges) }
    }