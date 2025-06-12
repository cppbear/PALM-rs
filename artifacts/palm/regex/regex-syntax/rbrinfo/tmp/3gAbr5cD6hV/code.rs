pub fn new<I>(ranges: I) -> ClassUnicode
    where I: IntoIterator<Item=ClassUnicodeRange>
    {
        ClassUnicode { set: IntervalSet::new(ranges) }
    }