fn clone(&self) -> ReseedingCore<R, Rsdr> {
        ReseedingCore {
            inner: self.inner.clone(),
            reseeder: self.reseeder.clone(),
            threshold: self.threshold,
            bytes_until_reseed: 0, // reseed clone on first use
        }
    }