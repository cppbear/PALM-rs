pub fn new(pattern: &str) -> RegexBuilder {
        let mut builder = RegexBuilder(RegexOptions::default());
        builder.0.pats.push(pattern.to_owned());
        builder
    }