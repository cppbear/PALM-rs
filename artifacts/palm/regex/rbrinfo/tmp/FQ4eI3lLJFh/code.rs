fn from_str(s: &str) -> Result<Regex, Error> {
        Regex::new(s)
    }