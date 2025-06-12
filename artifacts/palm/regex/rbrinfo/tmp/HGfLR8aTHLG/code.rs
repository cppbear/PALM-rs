pub fn into_byte_regex_set(self) -> re_set::bytes::RegexSet {
        re_set::bytes::RegexSet::from(self)
    }