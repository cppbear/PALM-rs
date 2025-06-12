pub fn new<I, S>(exprs: I) -> Result<RegexSet, Error>
            where S: AsRef<str>, I: IntoIterator<Item=S> {
        RegexSetBuilder::new(exprs).build()
    }