fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("MapDeserializer")
            .field("iter", &self.iter)
            .field("value", &self.value)
            .field("count", &self.count)
            .finish()
    }