fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("SeqDeserializer")
            .field("iter", &self.iter)
            .field("count", &self.count)
            .finish()
    }