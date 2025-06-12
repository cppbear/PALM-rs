pub fn into_regex_set(self) -> re_set::unicode::RegexSet {
        re_set::unicode::RegexSet::from(self)
    }