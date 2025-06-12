pub fn finish(&mut self) -> Result<W> {
        // If we could consume self in finish(), we wouldn't have to worry about this case, but
        // finish() is retryable in the face of I/O errors, so we can't consume here.
        assert!(
            self.delegate.is_some(),
            "Encoder has already had finish() called"
        );

        self.write_final_leftovers()?;

        let writer = self.delegate.take().expect("Writer must be present");

        Ok(writer)
    }