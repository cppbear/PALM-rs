pub fn from_str(s: &'a str) -> Self {
        Deserializer::new(read::StrRead::new(s))
    }