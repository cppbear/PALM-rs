pub fn new(re: &str) -> Result<Regex, Error> {
        RegexBuilder::new(re).build()
    }