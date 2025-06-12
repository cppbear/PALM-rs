pub fn into_inner(mut self) -> W {
        self.delegate
            .take()
            .expect("Encoder has already had finish() called")
    }