pub fn new(threshold: u64, reseeder: Rsdr) -> Result<Self, Rsdr::Error> {
        Ok(ReseedingRng(BlockRng::new(ReseedingCore::new(
            threshold, reseeder,
        )?)))
    }