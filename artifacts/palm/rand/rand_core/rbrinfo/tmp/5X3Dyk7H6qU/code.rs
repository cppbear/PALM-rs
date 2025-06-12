fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockRng")
            .field("core", &self.core)
            .field("result_len", &self.results.as_ref().len())
            .field("index", &self.index)
            .finish()
    }