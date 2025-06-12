fn flush(&mut self) -> Result<()> {
        self.write_all_encoded_output()?;
        self.delegate
            .as_mut()
            .expect("Writer must be present")
            .flush()
    }