fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "variant of enum {}", self.enum_name)
    }