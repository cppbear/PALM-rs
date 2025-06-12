pub fn into_regex(self) -> re_unicode::Regex {
        re_unicode::Regex::from(self)
    }