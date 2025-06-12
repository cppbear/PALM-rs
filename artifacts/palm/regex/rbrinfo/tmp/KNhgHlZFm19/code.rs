fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegexSet({:?})", self.0.regex_strings())
    }