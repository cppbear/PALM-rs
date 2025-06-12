fn write_all_encoded_output(&mut self) -> Result<()> {
        while self.output_occupied_len > 0 {
            let remaining_len = self.output_occupied_len;
            match self.write_to_delegate(remaining_len) {
                // try again on interrupts ala write_all
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                // other errors return
                Err(e) => return Err(e),
                // success no-ops because remaining length is already updated
                Ok(()) => {}
            };
        }

        debug_assert_eq!(0, self.output_occupied_len);
        Ok(())
    }