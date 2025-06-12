fn write_final_leftovers(&mut self) -> Result<()> {
        if self.delegate.is_none() {
            // finish() has already successfully called this, and we are now in drop() with a None
            // writer, so just no-op
            return Ok(());
        }

        self.write_all_encoded_output()?;

        if self.extra_input_occupied_len > 0 {
            let encoded_len = self
                .engine
                .encode_slice(
                    &self.extra_input[..self.extra_input_occupied_len],
                    &mut self.output[..],
                )
                .expect("buffer is large enough");

            self.output_occupied_len = encoded_len;

            self.write_all_encoded_output()?;

            // write succeeded, do not write the encoding of extra again if finish() is retried
            self.extra_input_occupied_len = 0;
        }

        Ok(())
    }