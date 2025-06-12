pub fn into_byte_regex(self) -> re_bytes::Regex {
        re_bytes::Regex::from(self)
    }