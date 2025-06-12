fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Deserializer::from_str(s)
            .parse_any_signed_number()
            .map(Into::into)
    }