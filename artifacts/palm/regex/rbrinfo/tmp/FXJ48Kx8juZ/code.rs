pub fn new<I, S>(patterns: I) -> RegexSetBuilder
            where S: AsRef<str>, I: IntoIterator<Item=S> {
        let mut builder = RegexSetBuilder(RegexOptions::default());
        for pat in patterns {
            builder.0.pats.push(pat.as_ref().to_owned());
        }
        builder
    }