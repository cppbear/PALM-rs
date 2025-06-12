fn write_to_delegate(&mut self, current_output_len: usize) -> Result<()> {
        self.panicked = true;
        let res = self
            .delegate
            .as_mut()
            .expect("Writer must be present")
            .write(&self.output[..current_output_len]);
        self.panicked = false;

        res.map(|consumed| {
            debug_assert!(consumed <= current_output_len);

            if consumed < current_output_len {
                self.output_occupied_len = current_output_len.checked_sub(consumed).unwrap();
                // If we're blocking on I/O, the minor inefficiency of copying bytes to the
                // start of the buffer is the least of our concerns...
                // TODO Rotate moves more than we need to; copy_within now stable.
                self.output.rotate_left(consumed);
            } else {
                self.output_occupied_len = 0;
            }
        })
    }