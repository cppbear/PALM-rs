fn clone(&self) -> ReseedingRng<R, Rsdr> {
        // Recreating `BlockRng` seems easier than cloning it and resetting
        // the index.
        ReseedingRng(BlockRng::new(self.0.core.clone()))
    }