pub fn into_inner(mut self) -> S {
        self.encoder
            .finish()
            .expect("Writing to a consumer should never fail")
            .str_consumer
    }