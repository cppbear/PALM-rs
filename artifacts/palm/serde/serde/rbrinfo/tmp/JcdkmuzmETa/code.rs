fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "enum variant cannot be serialized: {:?}", self.0)
    }