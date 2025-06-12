fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockRng64")
            .field("core", &self.core)
            .field("result_len", &self.results.as_ref().len())
            .field("index", &self.index)
            .field("half_used", &self.half_used)
            .finish()
    }